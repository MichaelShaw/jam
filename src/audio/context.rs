use alto;
use alto::{Context, StaticSource, StreamingSource, Buffer, SourceTrait};
use alto::{Mono, Stereo};

use std::sync::Arc;
use std::path::{PathBuf};

use super::{Gain, SoundEvent, SoundName, DistanceModel};
use super::load::{load_combined, LoadedSound};
use super::source::{Sources, SoundSource, StreamingSoundSource, SoundSourceLoan, assign_event_static};

use std::fs::File;
use lewton::inside_ogg::OggStreamReader;

use Vec3f;
use HashMap;
use JamResult;
use JamError;
use cgmath::Zero;

pub struct SoundContext<'d> {
    pub context: &'d Context<'d>,
    pub path: String,
    pub extension: String,
    pub sources: Sources<'d>,
    pub buffers: HashMap<SoundName, SoundBuffer<'d>>,
    pub stream_above_file_size: u64,
    pub master_gain : Gain,
    pub distance_model : DistanceModel,
    pub listener : Listener,
}

pub struct SoundBuffer<'d> {
    pub inner : Arc<Buffer<'d, 'd>>,
    pub gain: Gain,
    pub duration: f32, // we could track last used .... could be interesting if nothing else
}



#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Listener {
    pub position: Vec3f,
    pub velocity: Vec3f,
    pub orientation_up: Vec3f,
    pub orientation_forward: Vec3f,
}

impl Listener {
    pub fn default() -> Listener {
        Listener {
            position: Vec3f::zero(),
            velocity: Vec3f::zero(),
            orientation_up: Vec3f::new(0.0, 1.0, 0.0),
            orientation_forward: Vec3f::new(0.0, 0.0, -1.0),
        }
    }
}

pub fn create_sound_context<'d>(context: &'d Context<'d>, path:&str, extension: &str, stream_above_file_size: u64) -> SoundContext<'d> {
    // we should probably create our sources here
    SoundContext {
        context: context,
        path: String::from(path),
        extension: String::from(extension),
        sources: Sources {
            next_event: 0,
            sources: Vec::new(),
            streaming: Vec::new(),
        },
        buffers: HashMap::default(),
        stream_above_file_size: stream_above_file_size,
        master_gain: 1.0,
        distance_model: alto::DistanceModel::None,
        listener: Listener::default() ,
    }
}

impl<'d> SoundContext<'d> {
    pub fn set_gain(&mut self, gain: Gain) -> JamResult<()> {
        try!(self.context.set_gain(gain));
        self.master_gain = gain;
        Ok(())
    }

    pub fn create(&mut self, static_count: usize, streaming_count: usize) -> JamResult<()> {
        for _ in 0..static_count {
            let source = try!(self.context.new_static_source());
            self.sources.sources.push(SoundSource { inner: source, current_binding: None});
        }
        for _ in 0..streaming_count {
            let source = try!(self.context.new_streaming_source());
            self.sources.streaming.push(StreamingSoundSource { inner: source, stream_reader: None, current_binding: None });
        }
        Ok(())
    }

    pub fn set_listener(&mut self, listener: Listener) -> JamResult<()> {
        try!(self.context.set_position(listener.position));
        try!(self.context.set_velocity(listener.velocity));
        try!(self.context.set_orientation::<[f32; 3]>((listener.orientation_forward.into(), listener.orientation_up.into())));

        self.listener = listener;
        
        Ok(())
    }

     pub fn purge(&mut self) -> JamResult<()> {
        try!(self.sources.purge());
        self.buffers.clear();
        Ok(())
    }

    pub fn full_path(&self, name: &str) -> PathBuf {
        PathBuf::from(format!("{}/{}.{}", &self.path, name, &self.extension))
    }


    pub fn set_distace_model(&mut self, distance_model: DistanceModel) -> JamResult<()> {
        try!(self.context.set_distance_model(distance_model));
        self.distance_model = distance_model;
        Ok(())
    }

    // just convenience
    pub fn stop(&mut self, loan:SoundSourceLoan) -> JamResult<()> {
        if let Some(ref mut source) = self.sources.for_loan(loan) {
            try!(source.stop());
        }
        Ok(())
    }

    pub fn play_event(&mut self, sound_event: SoundEvent, loan: Option<SoundSourceLoan>) -> JamResult<SoundSourceLoan> {
        if let Some(l) = loan {
            if let Some(mut s) = self.sources.for_loan(l) {
                try!(s.assign_event(sound_event, l.event_id));
                return Ok(l)
            }
        } 
        
        if let Some(buffer) = self.buffers.get(&sound_event.name) {
            // sound is loaded
            if let Some((ref mut source, loan)) = self.sources.loan_next_free_static() {
                // and there's a free source
                try!(source.inner.set_buffer(buffer.inner.clone()));
                try!(assign_event_static(source, sound_event, loan.event_id));
                try!(source.inner.play());
     
                Ok(loan)
            } else {
                Err(JamError::NoFreeSource(false))
            }
        } else {
            // ok we need to load/stream it
            let combined_load = try!(load_combined(self.full_path(&sound_event.name), self.stream_above_file_size));

            // we need to call out here ...
            match combined_load {
                LoadedSound::Static(sound) => {
                    
                     Err(JamError::NoFreeSource(true))    
                },
                LoadedSound::Streaming(ogg_stream_reader) => {
                    return if let Some((source, loan)) = self.sources.loan_next_free_streaming() {
                        Err(JamError::NoFreeSource(true))
                    } else {
                        Err(JamError::NoFreeSource(true))
                    };
                },
            }
        }
    }
}
