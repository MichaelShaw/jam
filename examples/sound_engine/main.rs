#![allow(dead_code)]

extern crate jam;
extern crate alto;

use alto::Alto;
// use jam::audio::*;
use jam::audio::engine::{process};
use jam::audio::engine::SoundEngineUpdate::*;

fn main() {
    let alto = Alto::load("./OpenAL64.dll").unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();
    let mut cb = jam::audio::context::create_sound_context(&ctx, "resources/sound", "ogg");
    cb.create_sources(32, 4).unwrap();

    let sounds = vec![("teleport".into(), 1.0), ("water".into(), 1.0)];
    process(&mut cb, Preload(sounds));
}