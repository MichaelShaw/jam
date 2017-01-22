pub mod engine;

use alto;

use {HashSet, HashMap};
use Vec3;

use self::engine::{SoundEvent, Listener, DistanceModel};

pub enum SoundEngineUpdate {
    Preload(HashSet<String>), // load buffers
    DistanceModel(DistanceModel),
    Render { master_gain: f32, sounds:Vec<SoundEvent>, persistent_sounds:HashMap<String, SoundEvent>, listener: Listener },
    Shutdown, // unbind all sources, destroy all buffers
}

// blend speed for persistent sounds, in, out?

