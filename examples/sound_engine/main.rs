#![allow(dead_code)]

extern crate jam;
extern crate alto;
extern crate cgmath;

use alto::Alto;
// use jam::audio::*;
use jam::audio::{Listener, SoundEvent};
use jam::audio::engine::{process};
use jam::audio::engine::SoundEngineUpdate::*;
use jam::HashMap;
use jam::Vec3f;
use cgmath::Zero;

#[cfg(target_os = "windows")] 
const OPENAL_PATH: &'static str = "./native/windows/OpenAL64.dll";
#[cfg(target_os = "macos")]
const OPENAL_PATH: &'static str = "./native/mac/openal.dylib";

fn main() {
    let alto = Alto::load(OPENAL_PATH).unwrap();
    let dev = alto.open(None).unwrap();
    let ctx = dev.new_context(None).unwrap();
    let mut cb = jam::audio::context::create_sound_context(&ctx, "resources/sound", "ogg");
    cb.create_sources(32, 4).unwrap();

    let sounds = vec![("teleport".into(), 1.0), ("water".into(), 1.0)];
    process(&mut cb, Preload(sounds)).unwrap();

    let listener = Listener::default();

    let sound_event = SoundEvent {
        name: "teleport".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.5,
        attenuation:1.0,
    };
    let sound_event_b = SoundEvent {
        name: "water".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.0,
        attenuation:1.0,
    };

    process(&mut cb, Render { master_gain: 1.0, sounds:vec![sound_event, sound_event_b], persistent_sounds:HashMap::default(), listener: listener }).unwrap();

    std::thread::sleep(std::time::Duration::new(2, 0));

    process(&mut cb, Clear).unwrap();

    std::thread::sleep(std::time::Duration::new(1, 0));
}