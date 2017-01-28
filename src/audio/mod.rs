pub mod engine;
pub mod load;
pub mod context;
pub mod source;

use alto;

use Vec3f;

// blend speed for persistent sounds, in, out?

pub type SoundName = String;

pub type SoundEventId = u64; 

pub type Gain = f32;

pub type DistanceModel = alto::DistanceModel;

#[derive(Clone, Debug)]
pub struct SoundEvent {
    pub name: String,
    pub position: Vec3f,
    pub gain: f32,
    pub pitch: f32,
    pub attenuation: f32, // unsure if this should be bool for relative, or an optional rolloff factor (within the context distance model)
    pub loop_sound: bool,
}

pub type Listener = self::context::Listener;