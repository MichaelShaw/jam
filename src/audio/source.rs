use alto::{Context, StaticSource, StreamingSource, Buffer, SourceTrait};
use JamResult;
use Vec3f;
use std::fs::File;

use lewton::inside_ogg::OggStreamReader;

use super::*;

pub struct SoundSource<'d> {
    pub inner: StaticSource<'d, 'd>, // make this private at some point?
    pub current_binding: Option<SoundBinding>,
}

pub struct StreamingSoundSource<'d> {
    pub inner: StreamingSource<'d, 'd>, // make this private at some point?
    pub stream_reader : Option<OggStreamReader<File>>,
    pub current_binding: Option<SoundBinding>,
}

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
        let len = self.sources.len();
        for i in 0..len {
            let source = &mut self.sources[i];
            let loan = SoundSourceLoan {
                source_id: i,
                event_id: event_id,
                streaming: false,
            };
            return Some((source, loan));
        }

        None
    }

    pub fn loan_next_free_streaming<'a>(&'a mut self) -> Option<(&'a mut StreamingSoundSource<'d>, SoundSourceLoan)> {
        let event_id = self.next_event_id();
        let len = self.streaming.len();
        for i in 0..len {
            let source = &mut self.streaming[i];
            let loan = SoundSourceLoan {
                source_id: i,
                event_id: event_id,
                streaming: true,
            };
            return Some((source, loan));
        }

        None
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

// these perhaps should be implemented on their respective sources
pub fn assign_event_static(source: &mut SoundSource, sound_event: SoundEvent, event_id: SoundEventId) -> JamResult<()> {
    try!(assign_event_details(&mut source.inner, &sound_event));
    try!(source.inner.set_looping(sound_event.loop_sound));
    source.current_binding = Some(SoundBinding {
        event_id: event_id,
        sound_event: sound_event,
    });
    Ok(())
}

pub fn assign_event_streaming(source: &mut StreamingSoundSource, sound_event: SoundEvent, event_id: SoundEventId) -> JamResult<()> {
    try!(assign_event_details(&mut source.inner, &sound_event));
    source.current_binding = Some(SoundBinding {
        event_id: event_id,
        sound_event: sound_event,
    });
    Ok(())
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
                try!(assign_event_static(source, event, event_id));
            },
            &mut Streaming(ref mut source) => {
                try!(assign_event_streaming(source, event, event_id));
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
                    println!("unqueing buffer for stream!!");
                    try!(source.inner.unqueue_buffer());
                }
            },
        }
        Ok(())
    }
}