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
        DistanceModel(model) => {
            try!(context.set_distace_model(model))
        },
        Render { master_gain, sounds, persistent_sounds, listener } => {
            if context.master_gain != master_gain {
                println!("updating master gain to {:?}", master_gain);
                try!(context.set_gain(master_gain));
            }
            if context.listener != listener {
                println!("updating listener!");
                try!(context.set_listener(listener));
            }
            if !sounds.is_empty() {
                try!(context.clean_sources()); // a bit eager, but what the hell
            }
            for sound_event in sounds {
                try!(context.play_event(sound_event));
            }
            if !persistent_sounds.is_empty() {
                println!("got persistent sounds too -> {:?}", persistent_sounds);
            }

            ()   
        },
        Clear => {
            try!(context.purge());
            ()
        },
    };
    Ok(())
}