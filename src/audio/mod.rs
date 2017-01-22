

use alto::{Alto, Context, StaticSource, Buffer, SourceTrait};
// use ogg;
// use lewton;
use HashMap;
use std::sync::Arc;
use std::path::{PathBuf};
use Vec3;

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: PathBuf,
    pub sources: Vec<SoundSource<'a>>, // THESE ARE REFERENCES TO SELF, THIS IS ILLEGAL
    pub buffers: HashMap<String, Arc<Buffer<'a, 'a>>>,
    pub next_event : SoundEventId,
}

pub type SoundEventId = u64; 

pub struct SoundSourceLoan {
    pub source_id : usize,
    pub event_id : SoundEventId,
}

pub struct SoundSource<'a> {
    static_source: StaticSource<'a, 'a>,
    pub current_event: Option<SoundEventId>,
}

#[derive(Copy, Clone)]
pub struct ListenerInformation {
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation_up: Vec3,
    pub orientation_forward: Vec3,
}

pub fn create_sound_context<'a>(context: &'a Context<'a>, path:&str) -> SoundContext<'a> {
    SoundContext {
        context: context,
        path: PathBuf::from(path),
        sources: Vec::new(),
        buffers: HashMap::default(),
        next_event: 0,
    }
}

pub fn do_it() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();

    let mut cb = create_sound_context(&ctx, "resources/sounds");
    let source = cb.context.new_static_source().unwrap();
    cb.sources.push(SoundSource { static_source: source, current_event: None});

    let buffer = Arc::new(cb.context.new_buffer().unwrap());
    cb.buffers.insert("bullshit".into(), buffer);

    if let Some(source) = cb.sources.first_mut() {
        if let Some(bb) = cb.buffers.get("bullshit") {
            println!("we have bullshit");
            source.static_source.set_buffer(Some(bb.clone()));
        }
        source.static_source.play();
        
    }



}   