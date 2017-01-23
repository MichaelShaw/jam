use std::path::{PathBuf};

use {HashMap, HashSet};
use Vec3;

use super::context::{DistanceModel, SoundEvent, Listener, SoundName};

pub enum SoundEngineUpdate {
    Preload(HashSet<SoundName>), // load buffers
    DistanceModel(DistanceModel),
    Render { master_gain: f32, sounds:Vec<SoundEvent>, persistent_sounds:HashMap<String, SoundEvent>, listener: Listener },
    Shutdown, // unbind all sources, destroy all buffers
}
