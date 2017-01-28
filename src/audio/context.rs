use alto;
use alto::{Context, Buffer, SourceTrait};
use alto::{Mono, Stereo};

use std::sync::Arc;
use std::path::{PathBuf};

use super::{Gain, SoundEvent, SoundName, DistanceModel};
use super::load::{load_combined, LoadedSound, load_ogg};
use super::source::{Sources, SoundSource, StreamingSoundSource, SoundSourceLoan};

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

    pub fn load_sound(&mut self, sound_name: &str, gain: Gain) -> JamResult<()> {
        let path = self.full_path(sound_name);
        let sound = try!(load_ogg(path));
        let mut buffer = try!(self.context.new_buffer());
        let duration = sound.duration();
        if sound.channels == 1 {
            try!(buffer.set_data::<Mono<i16>, _>(sound.data, sound.sample_rate as i32));
        } else if sound.channels == 2 {
            try!(buffer.set_data::<Stereo<i16>, _>(sound.data, sound.sample_rate as i32));
        } else {
            return Err(JamError::TooManyChannels);
        }

        let arc_buffer = Arc::new(buffer);
        self.buffers.insert(sound_name.into(), SoundBuffer{ inner: arc_buffer, gain: gain, duration: duration });

        Ok(())
    }

    pub fn play_event(&mut self, sound_event: SoundEvent, loan: Option<SoundSourceLoan>) -> JamResult<SoundSourceLoan> {
        if let Some(l) = loan {
            if let Some(mut s) = self.sources.for_loan(l) {
                // we have a loan, just apply the event
                println!("we have loadn for {:?}", sound_event);
                try!(s.assign_event(sound_event, l.event_id));
                return Ok(l)
            }
        } 
        
        if let Some(buffer) = self.buffers.get(&sound_event.name) {
            // sound is loaded
            return if let Some((ref mut source, loan)) = self.sources.loan_next_free_static() {
                // println!("we have a sound event {:?} and now a loan {:?}", sound_event, loan);
                // and there's a free source
                try!(source.inner.set_buffer(buffer.inner.clone()));
                try!(source.assign_event(sound_event, loan.event_id));
                try!(source.inner.play());
     
                Ok(loan)
            } else {
                Err(JamError::NoFreeSource(false))
            }
        }

        // ok we need to load/stream it
        let combined_load = try!(load_combined(self.full_path(&sound_event.name), self.stream_above_file_size));

        // we need to call out here ...
        match combined_load {
            LoadedSound::Static(sound) => {
                let mut buffer = try!(self.context.new_buffer());
                let duration = sound.duration();
                if sound.channels == 1 {
                    try!(buffer.set_data::<Mono<i16>, _>(sound.data, sound.sample_rate as i32));
                } else if sound.channels == 2 {
                    try!(buffer.set_data::<Stereo<i16>, _>(sound.data, sound.sample_rate as i32));
                } else {
                    return Err(JamError::TooManyChannels);
                }

                let arc_buffer = Arc::new(buffer);
        
                let sound_name = sound_event.name.clone();
                
                let result = if let Some((source, loan)) = self.sources.loan_next_free_static() {
                    try!(source.inner.set_buffer(arc_buffer.clone()));
                    try!(source.assign_event(sound_event, loan.event_id));
                    try!(source.inner.play());
                    Ok(loan)
                } else {
                    Err(JamError::NoFreeSource(false))
                };
                self.buffers.insert(sound_name, SoundBuffer{ inner: arc_buffer, gain: 1.0, duration: duration });
                result
            },
            LoadedSound::Streaming(ogg_stream_reader) => {
                return if let Some((source, loan)) = self.sources.loan_next_free_streaming() {
                    source.stream_reader = Some(ogg_stream_reader);

                    try!(source.ensure_buffers_current(self.context));
                    try!(source.assign_event(sound_event, loan.event_id));
                    try!(source.inner.play());

                    println!("ok new streaming binding -> {:?}", source.current_binding);

                    Ok(loan)
                } else {
                    Err(JamError::NoFreeSource(true))
                };
            },
        }
    }

    pub fn ensure_buffers_current(&mut self) -> JamResult<()> {
        for source in self.sources.streaming.iter_mut() {
            println!("ok checking a streaming source");
            if source.current_binding.is_some() {
                println!("source has a current binding");
                try!(source.ensure_buffers_current(self.context));
            }
        }
        Ok(())
    }
}
