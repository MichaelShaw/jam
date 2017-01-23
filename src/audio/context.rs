use alto;
use alto::{Context, StaticSource, StreamingSource, Buffer, SourceTrait};
use alto::{Mono, Stereo};

use std::sync::Arc;
use std::path::{PathBuf};

use super::load::load_ogg;

use Vec3;
use HashMap;
use JamResult;
use JamError;

pub type SoundName = String;

pub type SoundEventId = u64; 

pub type Gain = f32;

pub type DistanceModel = alto::DistanceModel;

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: PathBuf,
    pub extension: String,
    pub sources: Vec<SoundSource<'a>>, 
    pub streaming_sources: Vec<StreamingSoundSource<'a>>,
    pub buffers: HashMap<SoundName, SoundBuffer<'a>>,
    pub next_event : SoundEventId,
    pub master_gain : Gain,
    pub listener : Listener,
}

pub struct SoundBuffer<'a> {
    pub inner : Arc<Buffer<'a, 'a>>,
    pub gain: Gain,
    pub duration: f32,
}

// an index to a source + binding
pub struct SoundSourceLoan {
    pub source_id : usize,
    pub event_id : SoundEventId,
}

pub struct SoundSource<'a> {
    static_source: StaticSource<'a, 'a>,
    pub current_event: Option<SoundBinding>,
}

pub struct StreamingSoundSource<'a> {
    streaming_source: StreamingSource<'a, 'a>
}

pub struct SoundBinding {
    pub event_id: SoundEventId,
    pub sound_event: SoundEvent,
}

#[derive(Clone)]
pub struct SoundEvent {
    pub name: String,
    pub position: Vec3,
    pub gain: f32,
    pub pitch: f32,
    pub attenuation: f32,
}

#[derive(Copy, Clone)]
pub struct Listener {
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation_up: Vec3,
    pub orientation_forward: Vec3,
}

pub fn create_sound_context<'a>(context: &'a Context<'a>, path:&str, extension: &str) -> SoundContext<'a> {
    // we should probably create our sources here
    use cgmath::prelude::Zero;
    SoundContext {
        context: context,
        path: PathBuf::from(path),
        extension: String::from(extension),
        sources: Vec::new(),
        streaming_sources: Vec::new(),
        buffers: HashMap::default(),
        next_event: 0,
        master_gain: 1.0,
        listener: Listener {
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            orientation_up: Vec3::new(0.0, 1.0, 0.0),
            orientation_forward: Vec3::new(0.0, 0.0, -1.0),
        },
    }
}

impl<'a> SoundContext<'a> {
    pub fn create_sources(&mut self, static_sources: usize, streaming_sources: usize) -> JamResult<()> {
        for _ in 0..static_sources {
            let source = try!(self.context.new_static_source().map_err(JamError::Alto));
            self.sources.push(SoundSource { static_source: source, current_event: None});
        }

        for _ in 0..streaming_sources {
            let source = try!(self.context.new_streaming_source().map_err(JamError::Alto));
            self.streaming_sources.push(StreamingSoundSource { streaming_source: source});
        }

        Ok(())
    }

    pub fn load_sound(&mut self, name: &str, gain: f32) -> JamResult<()> {
        let mut full_path = self.path.clone();
        full_path.push(name);
        full_path.set_extension(&self.extension);

        if full_path.exists() {
            let sound = try!(load_ogg(&full_path).map_err(JamError::Vorbis));
            let mut buffer = try!(self.context.new_buffer().map_err(JamError::Alto));
        
            let duration = sound.duration();
            if sound.channels == 1 {
                try!(buffer.set_data::<Mono<i16>, _>(sound.data, sound.sample_rate as i32).map_err(JamError::Alto));
            } else if sound.channels == 2 {
                try!(buffer.set_data::<Stereo<i16>, _>(sound.data, sound.sample_rate as i32).map_err(JamError::Alto));
            } else {
                return Err(JamError::TooManyChannels);
            }
               
            let arc_buffer = Arc::new(buffer);
            
            self.buffers.insert(name.into(), SoundBuffer{ inner: arc_buffer, gain: gain, duration: duration });

            Ok(())    
        } else {
            Err(JamError::FileDoesntExist(full_path))
        }
    }
}

/*

    let buffer = Arc::new(cb.context.new_buffer().unwrap());
    

    if let Some(source) = cb.sources.first_mut() {
        if let Some(bb) = cb.buffers.get("bullshit") {
            println!("we have bullshit");
            source.static_source.set_buffer(Some(bb.inner.clone()));
        }
        source.static_source.play();
        
    }*/