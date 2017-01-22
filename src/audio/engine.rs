use alto::{Alto, Device, Context, StaticSource, Buffer, SourceTrait};

use std::sync::Arc;
use std::path::{PathBuf};

use HashMap;
use Vec3;

pub type SoundEventId = u64; 

pub struct SoundContext<'a> {
    pub context: &'a Context<'a>,
    pub path: PathBuf,
    pub suffix: String,
    pub sources: Vec<SoundSource<'a>>, 
    pub buffers: HashMap<String, Arc<Buffer<'a, 'a>>>,
    pub next_event : SoundEventId,
    pub master_gain : f32,
}

pub struct SoundSourceLoan {
    pub source_id : usize,
    pub event_id : SoundEventId,
}

pub struct SoundSource<'a> {
    static_source: StaticSource<'a, 'a>,
    pub current_event: Option<SoundEventId>,
}

pub fn create_sound_context<'a>(context: &'a Context<'a>, path:&str, suffix: &str) -> SoundContext<'a> {
    // we should probably create our sources here
    SoundContext {
        context: context,
        path: PathBuf::from(path),
        suffix: String::from(suffix),
        sources: Vec::new(),
        buffers: HashMap::default(),
        next_event: 0,
        master_gain: 1.0,
    }
}

pub fn do_sample() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();

    let mut cb = create_sound_context(&ctx, "resources/sounds", "ogg");
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