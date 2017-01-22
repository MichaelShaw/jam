use alto;
use alto::{Alto, Device, Context, StaticSource, Buffer, SourceTrait};


use std::sync::Arc;
use std::path::{PathBuf};

use HashMap;
use Vec3;

pub type SoundName = String;

pub type SoundEventId = u64; 

pub type DistanceModel = alto::DistanceModel;

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: PathBuf,
    pub suffix: String,
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

pub fn create_sound_context<'a>(context: &'a Context<'a>, path:&str, suffix: &str) -> SoundContext<'a> {
    // we should probably create our sources here
    use cgmath::prelude::Zero;
    SoundContext {
        context: context,
        path: PathBuf::from(path),
        suffix: String::from(suffix),
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
    pub fn create_sources(&mut self, static_sources: usize, streaming_sources: usize) {
        for n in 0..static_sources {
            match self.context.new_static_source() {
                Ok(source) => {
                    println!("I got a source :D {:?}", n);
                    self.sources.push(SoundSource { static_source: source, current_event: None});
                }
                Err(error) => {
                    println!("error attempting to create source :-( {:?}", error);
                    break;
                },
            }
        }
    }
}

pub fn do_sample(static_sources: usize, streaming_sources: usize) {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();

    let mut cb = create_sound_context(&ctx, "resources/sounds", "ogg");

    cb.create_sources(static_sources, streaming_sources);

    let buffer = Arc::new(cb.context.new_buffer().unwrap());
    cb.buffers.insert("bullshit".into(), SoundBuffer{ inner: buffer, gain: 1.0, duration: 1.0 });

    if let Some(source) = cb.sources.first_mut() {
        if let Some(bb) = cb.buffers.get("bullshit") {
            println!("we have bullshit");
            source.static_source.set_buffer(Some(bb.inner.clone()));
        }
        source.static_source.play();
        
    }
}   