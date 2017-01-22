pub mod engine;

use alto;

use {HashSet, HashMap};
use Vec3;

#[derive(Copy, Clone)]
pub struct Listener {
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation_up: Vec3,
    pub orientation_forward: Vec3,
}

pub type DistanceModel = alto::DistanceModel;

pub enum SoundEngineEvent {
    Preload(HashSet<String>), // load buffers
    DistanceModel(DistanceModel),
    Render { master_gain: f32, effects:Vec<SoundEffect>, persistent_effects:HashMap<String, SoundEffect>, listener: Listener },
    Shutdown, // unbind all sources, destroy all buffers
}

#[derive(Clone)]
pub struct SoundEffect {
    pub name: String,
    pub position: Vec3,
    pub gain: f32,
    pub pitch: f32,
    pub attenuation: f32,
}