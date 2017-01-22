
use alto::{Alto, Device, Context, StaticSource, Buffer, SourceTrait};
// use ogg;
// use lewton;
use HashMap;
use std::sync::Arc;

pub struct DeviceBox<'a> {
    pub device: Device<'a>,
}

pub fn create_context() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None).unwrap();
    println!("I did stuff!");
}

pub fn create_device(alto: &Alto) -> DeviceBox {
    let dev = alto.open(None).unwrap();
    DeviceBox {
        device: dev
    }
}


pub struct ContextBox<'a> {
    pub device: &'a Device<'a>,
    pub context: &'a Context<'a>,
    pub sources: Vec<StaticSource<'a, 'a>>, // THESE ARE REFERENCES TO SELF, THIS IS ILLEGAL
    pub buffers: HashMap<String, Arc<Buffer<'a, 'a>>>,
}

pub fn mah_context<'a>(device: &'a Device<'a>, context: &'a Context<'a>) -> ContextBox<'a> {
    ContextBox {
        device: device,
        context: context,
        sources: Vec::new(),
        buffers: HashMap::default()
    }
}

pub fn do_it() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();

    let mut cb = mah_context(&dev, &ctx);
    let source = cb.context.new_static_source().unwrap();
    cb.sources.push(source);

    let buffer = Arc::new(cb.context.new_buffer().unwrap());
    cb.buffers.insert("bullshit".into(), buffer);

    if let Some(source) = cb.sources.first_mut() {
        if let Some(bb) = cb.buffers.get("bullshit") {
            source.set_buffer(Some(bb.clone()));
        }
        source.play();
        
    }



}   