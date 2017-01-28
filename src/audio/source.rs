use alto::{Context, StaticSource, StreamingSource, Buffer, SourceTrait, Mono, Stereo};
use JamResult;
use Vec3f;
use std::fs::File;

use lewton::inside_ogg::OggStreamReader;

use super::*;
use JamError;




// an index to a source + binding
#[derive(Debug, Clone, Copy)]
pub struct SoundSourceLoan {
    pub source_id: usize,
    pub event_id: SoundEventId,
    pub streaming: bool,
}

#[derive(Debug)]
pub struct SoundBinding {
    pub event_id: SoundEventId,
    pub sound_event: SoundEvent,
}

pub struct Sources<'d> {
    pub next_event: SoundEventId,
    pub sources: Vec<SoundSource<'d>>, 
    pub streaming: Vec<StreamingSoundSource<'d>>,
}

impl <'d> Sources<'d> {
    pub fn next_event_id(&mut self) -> SoundEventId {
        self.next_event += 1;
        self.next_event
    }

    pub fn next_free_static_idx(&self) -> Option<usize> {
        let len = self.sources.len();
        for i in 0..len {
            let source = &self.sources[i];
            if source.current_binding.is_none() {
                return Some(i);
            }
        }
        None
    }

    pub fn loan_next_free_static<'a>(&'a mut self) -> Option<(&'a mut SoundSource<'d>, SoundSourceLoan)> {
        let event_id = self.next_event_id();
        let first_free = self.sources.iter_mut().enumerate().find(|&(_, ref s)| s.current_binding.is_none());
        if let Some((idx, source)) = first_free {
            let loan = SoundSourceLoan {
                source_id: idx,
                event_id: event_id,
                streaming: false,
            };
            Some((source, loan))
        } else {
            None
        }
    }

    pub fn loan_next_free_streaming<'a>(&'a mut self) -> Option<(&'a mut StreamingSoundSource<'d>, SoundSourceLoan)> {
        let event_id = self.next_event_id();
        let first_free = self.streaming.iter_mut().enumerate().find(|&(_, ref s)| s.current_binding.is_none());
        if let Some((idx, source)) = first_free {
            let loan = SoundSourceLoan {
                source_id: idx,
                event_id: event_id,
                streaming: true,
            };
            Some((source, loan))
        } else {
            None
        }
    }

    // I don't really understand this 'a on the mut self :-(
    pub fn for_loan<'a>(&'a mut self, loan:SoundSourceLoan) -> Option<CombinedSource<'d, 'a>> {
        use self::CombinedSource::*;
        if loan.streaming {
            let mut source : &'a mut StreamingSoundSource<'d> = &mut self.streaming[loan.source_id];
            let valid = source.current_binding.iter().any(|ss| ss.event_id == loan.event_id );
            if valid {
                Some(Streaming(source))
            } else {
                None
            }
        } else {
            let mut source : &'a mut SoundSource<'d> = &mut self.sources[loan.source_id];
            let valid = source.current_binding.iter().any(|ss| ss.event_id == loan.event_id );
            if valid {
                Some(Static(source))
            } else {
                None
            }
        }
    }

    pub fn purge(&mut self) -> JamResult<()> {
        for source in self.sources.iter_mut() {
            if source.current_binding.is_some() {
                try!(source.inner.stop());
                source.current_binding = None;
            }
        }
        for source in self.streaming.iter_mut() {
            if source.current_binding.is_some() {
                try!(source.inner.stop());
                source.current_binding = None;
            }
        }
        Ok(())
    }
    
    // just updates book keeping of sources that have stopped since we checked (so we can throw away the binding)
    pub fn clean(&mut self) -> JamResult<(u32, u32)> {
        use alto::SourceState::*;

        let mut available_sources = 0;
        let mut available_streaming_sources = 0;

        for source in self.sources.iter_mut() {
            if source.current_binding.is_some() {
                let state = try!(source.inner.state());
                match state {
                    Initial | Playing | Paused => (),
                    Stopped => {
                        source.current_binding = None;
                        available_sources += 1;   
                    },
                };
            } else {
                available_sources += 1;
            }
        }
        for source in self.streaming.iter_mut() {
            if source.current_binding.is_some() {
                let state = try!(source.inner.state());
                match state {
                    Initial | Playing | Paused => (),
                    Stopped => {
                        source.current_binding = None;
                        available_streaming_sources += 1;   
                    },
                };
            } else {
                available_streaming_sources += 1;
            }
        }

        Ok((available_sources, available_streaming_sources))
    }
}


pub struct SoundSource<'d> {
    pub inner: StaticSource<'d, 'd>, // make this private at some point?
    pub current_binding: Option<SoundBinding>,
}

impl<'d> SoundSource<'d> {
    // these perhaps should be implemented on their respective sources
    pub fn assign_event(&mut self, sound_event: SoundEvent, event_id: SoundEventId) -> JamResult<()> {
        try!(assign_event_details(&mut self.inner, &sound_event));
        try!(self.inner.set_looping(sound_event.loop_sound));
        self.current_binding = Some(SoundBinding {
            event_id: event_id,
            sound_event: sound_event,
        });
        Ok(())
    }

    pub fn clean(&mut self) {
        self.current_binding = None;
    }
}

pub struct StreamingSoundSource<'d> {
    pub inner: StreamingSource<'d, 'd>, // make this private at some point?
    pub stream_reader : Option<OggStreamReader<File>>,
    pub current_binding: Option<SoundBinding>,
}

impl<'d> StreamingSoundSource<'d> {
    pub fn assign_event(&mut self, sound_event: SoundEvent, event_id: SoundEventId) -> JamResult<()> {
        try!(assign_event_details(&mut self.inner, &sound_event));
        self.current_binding = Some(SoundBinding {
            event_id: event_id,
            sound_event: sound_event,
        });
        Ok(())
    }

    pub fn ensure_buffers_current(&mut self, context: &'d Context<'d>) -> JamResult<()> {
        'main: loop {
            let queued = try!(self.inner.buffers_queued());
            let processed = try!(self.inner.buffers_processed());
            println!("queued count {:?}", queued);
            if queued < 5 || processed > 0 {
                println!("not enough buffers!");
                let cleanup : bool = if let Some(ref mut reader) = self.stream_reader {
                    // 1 for 1 is retarded
                    let channels = reader.ident_hdr.audio_channels;
                    let sample_rate = reader.ident_hdr.audio_sample_rate;
                    let mut data : Vec<i16> = Vec::new();
                    let eof : bool = match drain(reader, &mut data, 50_000) {
                        Ok(bytes_read) => bytes_read < 50_000,
                        Err(err) => true,
                    };

                    if data.len() > 0 {
                        println!("we read a packet :D");
                        let mut buffer : Buffer = if try!(self.inner.buffers_processed()) > 0 {
                            println!("dequeueing buffer");
                            try!(self.inner.unqueue_buffer())
                        } else {
                            println!("new buffer");
                            try!(context.new_buffer())
                        };

                           

                        let duration = (data.len() as f32) / (sample_rate as f32);
                        println!("duration of packet -> {:?}, data length {:?}", duration, data.len());
                        

                        if channels == 1 {
                            try!(buffer.set_data::<Mono<i16>, _>(data, sample_rate as i32));
                        } else if channels == 2 {
                            try!(buffer.set_data::<Stereo<i16>, _>(data, sample_rate as i32));
                        } else {
                            return Err(JamError::TooManyChannels);
                        }

                        match self.inner.queue_buffer(buffer) {
                            Ok(()) => {
                                println!("we queued a buffer");
                                eof
                            },
                            Err((error, _)) => {
                                println!("no queued buffer fml");   
                                return Err(JamError::Alto(error))
                            },
                        }
                    } else {
                        true
                    }
                } else {
                    println!("no stream reader to start with ...");
                    break 'main;
                };
                if cleanup {
                    println!("Destroying stream reader");
                    self.stream_reader = None;
                }
            } else {
                break 'main;
            }
        }
        Ok(())
    }

    pub fn clean(&mut self) -> JamResult<()> {
        self.stream_reader = None;
        self.current_binding = None;
        // drain some buffers as well
        Ok(())
    }
}

fn drain(reader: &mut OggStreamReader<File>, data: &mut Vec<i16>, samples: usize) -> JamResult<usize> {
    use std::iter::Extend;

    let mut samples_read : usize = 0;
    while data.len() < samples {
        if let Some(packet) = try!(reader.read_dec_packet_itl()) {
            samples_read += packet.len();
            data.extend(&packet);
        } else {
            break;
        }
    }

    Ok(samples_read)
}

pub fn assign_event_details<'d: 'c, 'c, ST : SourceTrait<'d,'c>>(source: &mut ST, sound_event:&SoundEvent) -> JamResult<()> {
    try!(source.set_pitch(sound_event.pitch));
    try!(source.set_position(sound_event.position));
    try!(source.set_gain(sound_event.gain));
    Ok(())
}

// used for retrieving loans
pub enum CombinedSource<'d: 'a, 'a> {
    Static(&'a mut SoundSource<'d>),
    Streaming(&'a mut StreamingSoundSource<'d>),
}

impl<'d: 'a, 'a> CombinedSource<'d, 'a> {
    pub fn assign_event(&mut self, event:SoundEvent, event_id: SoundEventId) -> JamResult<()> {
        use self::CombinedSource::*;
        match self {
            &mut Static(ref mut source) => {
                try!(source.assign_event(event, event_id));
            },
            &mut Streaming(ref mut source) => {
                try!(source.assign_event(event, event_id));
            },
        }
        Ok(())
    }

    pub fn stop(&mut self) -> JamResult<()> {
        use self::CombinedSource::*;
        match self {
            &mut Static(ref mut source) => {
                try!(source.inner.stop());
                try!(source.inner.clear_buffer());
                source.current_binding = None;
            },
            &mut Streaming(ref mut source) => {
                try!(source.inner.stop());
                source.stream_reader = None;
                source.current_binding = None;
                while try!(source.inner.buffers_processed()) > 0 {
                    try!(source.inner.unqueue_buffer());
                }
            },
        }
        Ok(())
    }
}