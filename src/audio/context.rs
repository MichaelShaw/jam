use alto;
use alto::{Context, StaticSource, StreamingSource, Buffer, SourceTrait};
use alto::{Mono, Stereo};

use std::sync::Arc;
use std::path::{PathBuf};

use super::load::load_ogg;

use std::fs::File;
use lewton::inside_ogg::OggStreamReader;

use Vec3f;
use HashMap;
use JamResult;
use JamError;
use cgmath::Zero;

pub type SoundName = String;

pub type SoundEventId = u64; 

pub type Gain = f32;

pub type DistanceModel = alto::DistanceModel;

pub struct SoundContext<'d> {
    pub context: &'d Context<'d>,
    pub path: String,
    pub extension: String,
    pub sources: Sources<'d>,
    pub buffers: HashMap<SoundName, SoundBuffer<'d>>,
    pub stream_above_file_size: u64,
    pub master_gain : Gain,
    pub distance_model : DistanceModel,
    pub listener : Listener,
}

pub struct SoundBuffer<'d> {
    pub inner : Arc<Buffer<'d, 'd>>,
    pub gain: Gain,
    pub duration: f32, // we could track last used .... could be interesting if nothing else
}

pub struct SoundSource<'d> {
    inner: StaticSource<'d, 'd>,
    pub current_binding: Option<SoundBinding>,
}

pub struct StreamingSoundSource<'d> {
    inner: StreamingSource<'d, 'd>,
    pub stream_reader : Option<OggStreamReader<File>>,
    pub current_binding: Option<SoundBinding>,
}

#[derive(Debug)]
pub struct SoundBinding {
    pub event_id: SoundEventId,
    pub sound_event: SoundEvent,
}

// an index to a source + binding
#[derive(Debug, Clone, Copy)]
pub struct SoundSourceLoan {
    pub source_id: usize,
    pub event_id: SoundEventId,
    pub streaming: bool,
}

#[derive(Clone, Debug)]
pub struct SoundEvent {
    pub name: String,
    pub position: Vec3f,
    pub gain: f32,
    pub pitch: f32,
    pub attenuation: f32,
    pub loop_sound: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Listener {
    pub position: Vec3f,
    pub velocity: Vec3f,
    pub orientation_up: Vec3f,
    pub orientation_forward: Vec3f,
}

impl Listener {
    pub fn default() -> Listener {
        Listener {
            position: Vec3f::zero(),
            velocity: Vec3f::zero(),
            orientation_up: Vec3f::new(0.0, 1.0, 0.0),
            orientation_forward: Vec3f::new(0.0, 0.0, -1.0),
        }
    }
}

pub fn create_sound_context<'d>(context: &'d Context<'d>, path:&str, extension: &str, stream_above_file_size: u64) -> SoundContext<'d> {
    // we should probably create our sources here
    SoundContext {
        context: context,
        path: String::from(path),
        extension: String::from(extension),
        sources: Sources {
            next_event: 0,
            sources: Vec::new(),
            streaming: Vec::new(),
        },
        buffers: HashMap::default(),
        stream_above_file_size: stream_above_file_size,
        master_gain: 1.0,
        distance_model: alto::DistanceModel::None,
        listener: Listener::default() ,
    }
}

impl<'d> SoundContext<'d> {
    pub fn set_gain(&mut self, gain: Gain) -> JamResult<()> {
        try!(self.context.set_gain(gain));
        self.master_gain = gain;
        Ok(())
    }

    pub fn create(&mut self, static_count: usize, streaming_count: usize) -> JamResult<()> {
        for _ in 0..static_count {
            let source = try!(self.context.new_static_source());
            self.sources.sources.push(SoundSource { inner: source, current_binding: None});
        }
        for _ in 0..streaming_count {
            let source = try!(self.context.new_streaming_source());
            self.sources.streaming.push(StreamingSoundSource { inner: source, stream_reader: None, current_binding: None });
        }
        Ok(())
    }

    pub fn set_listener(&mut self, listener: Listener) -> JamResult<()> {
        try!(self.context.set_position(listener.position));
        try!(self.context.set_velocity(listener.velocity));
        try!(self.context.set_orientation::<[f32; 3]>((listener.orientation_forward.into(), listener.orientation_up.into())));

        self.listener = listener;
        
        Ok(())
    }

     pub fn purge(&mut self) -> JamResult<()> {
        try!(self.sources.purge());
        self.buffers.clear();
        Ok(())
    }

    pub fn full_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}.{}", &self.path, name, &self.extension))
    }


    pub fn set_distace_model(&mut self, distance_model: DistanceModel) -> JamResult<()> {
        try!(self.context.set_distance_model(distance_model));
        self.distance_model = distance_model;
        Ok(())
    }

    // just convenience
    pub fn stop(&mut self, loan:SoundSourceLoan) -> JamResult<()> {
        if let Some(ref mut source) = self.sources.for_loan(loan) {
            try!(source.stop());
        }
        Ok(())
    }

    pub fn play_event(&mut self, sound_event: SoundEvent, loan: Option<SoundSourceLoan>) -> JamResult<SoundSourceLoan> {
        if let Some(l) = loan {
            if let Some(mut s) = self.sources.for_loan(l) {
                try!(s.assign_event(&sound_event));
                return Ok(l)
            }
        } 
        
        if let Some(buffer) = self.buffers.get(&sound_event.name) {
            // sound is loaded
            if let Some((source, loan)) = self.sources.loan_next_free_static() {
                // and there's a free source
                try!(source.inner.set_buffer(buffer.inner.clone()));
                try!(assign_event(&mut source.inner, &sound_event));
                try!(source.inner.play());

                Ok(loan)
            } else {
                Err(JamError::NoFreeSource)
            }
        } else {
            Ok(loan.unwrap())
        }
    }
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

// used for retrieving loans
pub enum CombinedSource<'d: 'a, 'a> {
    Static(&'a mut SoundSource<'d>),
    Streaming(&'a mut StreamingSoundSource<'d>),
}

impl<'d: 'a, 'a> CombinedSource<'d, 'a> {
    fn assign_event(&mut self, event:&SoundEvent) -> JamResult<()> {
        use self::CombinedSource::*;
        match self {
            &mut Static( ref mut source) => {
                try!(assign_event(&mut source.inner, event));
            },
            &mut Streaming(ref mut source) => {
                try!(assign_event(&mut source.inner, event));
            },
        }
        Ok(())
    }

    fn stop(&mut self) -> JamResult<()> {
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

fn assign_event<'d: 'c, 'c, ST : SourceTrait<'d,'c>>(source: &mut ST, sound_event:&SoundEvent) -> JamResult<()> {
    try!(source.set_pitch(sound_event.pitch));
    try!(source.set_position(sound_event.position));
    try!(source.set_gain(sound_event.gain));
    Ok(())
}

/*
pub position: Vec3f,
    pub gain: f32,
    pub pitch: f32,
    pub attenuation: f32,
    pub loop_sound: bool,*/