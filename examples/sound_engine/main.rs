#![allow(dead_code)]

#[macro_use]
extern crate jam;

extern crate alto;
extern crate cgmath;

use alto::Alto;
// use jam::audio::*;
use jam::audio::{Listener, SoundEvent};
use jam::audio::engine::{SoundEngine};
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
    let mut cb = jam::audio::context::create_sound_context(&ctx, "resources/sound", "ogg", 1_000_000);
    cb.create_sources(32, 4).unwrap();

    let listener = Listener::default();

    let sound_event = SoundEvent {
        name: "teleport".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.5,
        attenuation:1.0,
        loop_sound: false,
    };
    let sound_event_b = SoundEvent {
        name: "water".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.0,
        attenuation:1.0,
        loop_sound: false,
    };

    let persistent_sound = SoundEvent {
        name: "come.and.find.me".into(),
        position: Vec3f::zero(),
        gain: 1.0,
        pitch: 1.0,
        attenuation:1.0,
        loop_sound: false,
    };

    let mut engine = SoundEngine::new();

    engine.process(&mut cb, Preload(vec![("teleport".into(), 1.0), ("water".into(), 1.0)])).unwrap();

    engine.process(&mut cb, Render { master_gain: 1.0, sounds:vec![sound_event, sound_event_b], persistent_sounds: hashmap!["music".into() => persistent_sound], listener: listener }).unwrap();

    std::thread::sleep(std::time::Duration::new(2, 0));

    engine.process(&mut cb, Clear).unwrap();

    std::thread::sleep(std::time::Duration::new(1, 0));
}