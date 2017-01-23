use std::path::{PathBuf};

use {HashMap, JamResult};

use super::context::{DistanceModel, SoundEvent, Listener, SoundName, Gain, SoundContext};

pub enum SoundEngineUpdate {
    Preload(Vec<(SoundName, Gain)>), // load buffers
    DistanceModel(DistanceModel),
    Render { master_gain: f32, sounds:Vec<SoundEvent>, persistent_sounds:HashMap<String, SoundEvent>, listener: Listener },
    Clear, // unbind all sources, destroy all buffers
}

pub fn process(context: &mut SoundContext, update:SoundEngineUpdate) -> JamResult<()> {
    use self::SoundEngineUpdate::*;
    match update {
        Preload(sounds) => {
            for &(ref sound_name, gain) in &sounds {
                println!("preload {:?} gain {:?}", sound_name, gain);
                try!(context.load_sound(sound_name, gain));
            }
        },
        DistanceModel(ref model) => (),
        Render { master_gain, ref sounds, ref persistent_sounds, listener } => {
            ()   
        },
        Clear => (),
    };
    Ok(())
}