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

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: String,
    pub extension: String,
    pub sources: Vec<SoundSource<'a>>, 
    pub streaming_sources: Vec<StreamingSoundSource<'a>>,
    pub buffers: HashMap<SoundName, SoundBuffer<'a>>,
    pub stream_above_file_size: u64,
    pub next_event : SoundEventId,
    pub master_gain : Gain,
    pub distance_model : DistanceModel,
    pub listener : Listener,
}

pub struct SoundBuffer<'a> {
    pub inner : Arc<Buffer<'a, 'a>>,
    pub gain: Gain,
    pub duration: f32, // we could track last used .... could be interesting if nothing else
}

pub struct SoundSource<'a> {
    static_source: StaticSource<'a, 'a>,
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

pub struct StreamingSoundSource<'a> {
    streaming_source: StreamingSource<'a, 'a>,
    pub current_binding: Option<StreamingSoundBinding>,
}

pub struct StreamingSoundBinding {
    pub event_id: SoundEventId,
    pub sound_event: SoundEvent,
    pub stream_reader : OggStreamReader<File>,
    // details about how streaming is going?
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

pub fn create_sound_context<'a>(context: &'a Context<'a>, path:&str, extension: &str, stream_above_file_size: u64) -> SoundContext<'a> {
    // we should probably create our sources here
    SoundContext {
        context: context,
        path: String::from(path),
        extension: String::from(extension),
        sources: Vec::new(),
        streaming_sources: Vec::new(),
        buffers: HashMap::default(),
        stream_above_file_size: stream_above_file_size,
        next_event: 0,
        master_gain: 1.0,
        distance_model: alto::DistanceModel::None,
        listener: Listener::default() ,
    }
}

impl<'a> SoundContext<'a> {
    pub fn set_gain(&mut self, gain: Gain) -> JamResult<()> {
        try!(self.context.set_gain(gain).map_err(JamError::Alto));
        self.master_gain = gain;
        Ok(())
    }

    pub fn set_listener(&mut self, listener: Listener) -> JamResult<()> {
        try!(self.context.set_position(listener.position).map_err(JamError::Alto));
        try!(self.context.set_velocity(listener.velocity).map_err(JamError::Alto));
        try!(self.context.set_orientation::<[f32; 3]>((listener.orientation_forward.into(), listener.orientation_up.into())).map_err(JamError::Alto));

        self.listener = listener;
        
        Ok(())
    }

    pub fn create_sources(&mut self, static_sources: usize, streaming_sources: usize) -> JamResult<()> {
        for _ in 0..static_sources {
            let source = try!(self.context.new_static_source().map_err(JamError::Alto));
            self.sources.push(SoundSource { static_source: source, current_binding: None});
        }

        for _ in 0..streaming_sources {
            let source = try!(self.context.new_streaming_source().map_err(JamError::Alto));
            self.streaming_sources.push(StreamingSoundSource { streaming_source: source, current_binding: None });
        }

        Ok(())
    }

    pub fn purge(&mut self) -> JamResult<()> {
        for source in self.sources.iter_mut() {
            if source.current_binding.is_some() {
                try!(source.static_source.stop().map_err(JamError::Alto));
                source.current_binding = None;
            }
        }
        self.buffers.clear();
        Ok(())
    }

    pub fn full_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}.{}", &self.path, name, &self.extension))
    }

    pub fn load_sound(&mut self, name: &str, gain: f32) -> JamResult<()> {
        let full_path = self.full_path(name);
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

    pub fn next_event_id(&mut self) -> SoundEventId {
        self.next_event += 1;
        self.next_event
    }

    pub fn clean_sources(&mut self) -> JamResult<u32> {
        use alto::SourceState::*;

        let mut available_sources = 0;
        for source in self.sources.iter_mut() {
            if source.current_binding.is_some() {
                let state = try!(source.static_source.state().map_err(JamError::Alto));
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
        Ok(available_sources)
    }

    pub fn set_distace_model(&mut self, distance_model: DistanceModel) -> JamResult<()> {
        try!(self.context.set_distance_model(distance_model).map_err(JamError::Alto));
        self.distance_model = distance_model;
        Ok(())
    }

    pub fn play_event(&mut self, sound_event: SoundEvent, loan: Option<SoundSourceLoan>) -> JamResult<SoundSourceLoan> {
        if !self.buffers.contains_key(&sound_event.name) {
            try!(self.load_sound(&sound_event.name, sound_event.gain));
        }
        let event_id = self.next_event_id();
        if let Some(buffer) = self.buffers.get(&sound_event.name) {
            if let Some(source) = self.sources.iter_mut().filter(|src| src.current_binding.is_none()).next() {
                // fn set_buffer(&mut self, buf: Arc<Buffer<'d, 'c>
                try!(source.static_source.set_buffer(buffer.inner.clone()).map_err(JamError::Alto));
                try!(source.static_source.play().map_err(JamError::Alto));
                // next 3 are all generic over trait
                try!(source.static_source.set_pitch(sound_event.pitch).map_err(JamError::Alto));
                try!(source.static_source.set_position(sound_event.position).map_err(JamError::Alto));
                try!(source.static_source.set_gain(sound_event.gain).map_err(JamError::Alto));
                source.current_binding = Some(SoundBinding {
                    event_id: event_id,
                    sound_event: sound_event,
                });
                Ok(SoundSourceLoan {
                    source_id : 0,
                    event_id : event_id,
                    streaming: false,
                })
            } else {
                Err(JamError::NoFreeSource)
            }
        } else {
            Err(JamError::NoSound(sound_event.name))
        }
    }
}
