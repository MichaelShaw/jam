use {HashMap, JamResult};

use time;

use super::context::{DistanceModel, SoundEvent, SoundSourceLoan, Listener, SoundName, Gain, SoundContext};

#[derive(Debug, Clone)]
pub enum SoundEngineUpdate {
    Preload(Vec<(SoundName, Gain)>), // load buffers
    DistanceModel(DistanceModel),
    Render { master_gain: f32, sounds:Vec<SoundEvent>, persistent_sounds:HashMap<String, SoundEvent>, listener: Listener },
    Clear, // unbind all sources, destroy all buffers
}


// we need our state of what's already persisted, loans etc.

pub struct SoundEngine {
    // some measure of time
    // some notion of existing sounds
    pub last_render_time: u64,
    pub loans : HashMap<String, SoundSourceLoan>,
}

impl SoundEngine {
    pub fn new() -> SoundEngine {
        SoundEngine {
            last_render_time: time::precise_time_ns(),
            loans: HashMap::default(),
        }
    }

    pub fn process(&mut self, context: &mut SoundContext, update:SoundEngineUpdate) -> JamResult<()> {
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
                    try!(context.play_event(sound_event, None));
                }
                
                for (name, sound_event) in persistent_sounds {
                    let old_loan = self.loans.remove(&name);
                    let new_loan = try!(context.play_event(sound_event, old_loan));
                    self.loans.insert(name, new_loan);
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
}

/*
           if !self.buffers.contains_key(&sound_event.name) {
            println!("sound is missing, attemping to load -> {:?}", &sound_event.name);
            try!(self.load_sound(&sound_event.name, 1.0));
        }

        println!("playing -> {:?}", sound_event);

             

*/