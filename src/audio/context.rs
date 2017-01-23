use alto;
use alto::{Alto, Device, Context, StaticSource, Buffer, SourceTrait};

use std::sync::Arc;
use std::path::{PathBuf};

use Vec3;
use HashMap;
use JamResult;
use JamError;

pub type SoundName = String;

pub type SoundEventId = u64; 

pub type DistanceModel = alto::DistanceModel;

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: PathBuf,
    pub extension: String,
    pub sources: Vec<SoundSource<'a>>, 
    pub buffers: HashMap<SoundName, SoundBuffer<'a>>,
    pub next_event : SoundEventId,
    pub master_gain : f32,
    pub listener : Listener,
}

pub struct SoundBuffer<'a> {
    pub inner : Arc<Buffer<'a, 'a>>,
    pub gain: f32,
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

pub struct SoundBinding {
    pub event_id: SoundEventId,
    pub sound_Event: SoundEvent,
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

        Ok(())
    }

    pub fn load_sound(&mut self, path: &str) -> JamResult<()> {
        let mut full_path = self.path.clone();
        full_path.push(path);
        full_path.set_extension(&self.extension);

        println!("full path -> {:?}", full_path);
        if full_path.exists() {
            println!("path exists :D");
            Ok(())    
        } else {
            Err(JamError::FileDoesntExist(full_path))
        }
    }
}

/*

    let buffer = Arc::new(cb.context.new_buffer().unwrap());
    cb.buffers.insert("bullshit".into(), SoundBuffer{ inner: buffer, gain: 1.0, duration: 1.0 });

    if let Some(source) = cb.sources.first_mut() {
        if let Some(bb) = cb.buffers.get("bullshit") {
            println!("we have bullshit");
            source.static_source.set_buffer(Some(bb.inner.clone()));
        }
        source.static_source.play();
        
    }*/