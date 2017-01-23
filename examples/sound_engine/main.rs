#![allow(dead_code)]

extern crate jam;
extern crate alto;

use alto::Alto;
// use jam::audio::*;

fn main() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();
    let mut cb = jam::audio::context::create_sound_context(&ctx, "resources/sound", "ogg");
    cb.create_sources(32, 4).unwrap();
    cb.load_sound("teleport").expect("to load teleport sound");
    cb.load_sound("water").expect("to load water sound");

}