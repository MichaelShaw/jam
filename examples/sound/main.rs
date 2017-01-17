#![allow(dead_code)]

extern crate jam;

use alto::*;
use std::sync::Arc;

extern crate alto;
extern crate ogg;
extern crate lewton;

use lewton::VorbisError;
use lewton::inside_ogg::OggStreamReader;

use std::fs;
use std::fs::File;
use std::iter::Extend;

use std::path::{Path,PathBuf};

pub type SampleRate = u32;

#[derive(Clone, Debug)]
struct Sound {
    data : Vec<i16>,
    sample_rate: u32,
    channels: u8,
}

impl Sound {
    fn duration(&self) -> f32 {
        (self.data.len() as f32) / (self.sample_rate as f32)
    }
}

fn load_ogg<P: AsRef<Path>>(path: P) -> Result<Sound, VorbisError> {
    let f = try!(File::open(path));

	// Prepare the reading
    let packet_reader = ogg::PacketReader::new(f);
	let mut srr = try!(OggStreamReader::new(packet_reader));
    
    if srr.ident_hdr.audio_channels > 2 {
		// the openal crate can't process these many channels directly
        // std::vec::Vec<i16>
		println!("Stream error: {} channels are too many!", srr.ident_hdr.audio_channels);
	}

    // let mut len_play = 0.0;
    let mut data : Vec<i16> = Vec::new();
    while let Some(pck_samples) = try!(srr.read_dec_packet_itl()) {
        // println!("I got some shit {:?}", pck_samples);
        // len_play += pck_samples.len() as f32 / srr.ident_hdr.audio_sample_rate as f32;
        data.extend(pck_samples.iter());
    }
    
    Ok(Sound {
        data: data,
        sample_rate: srr.ident_hdr.audio_sample_rate,
        channels: srr.ident_hdr.audio_channels,
    })
}

fn main() {
//    let alto = Alto::load_default().unwrap();
    let alto = Alto::load("./OpenAL64.dll").unwrap();
 	println!("Using output: {:?}", alto.default_output().unwrap());

	let dev = alto.open(None).unwrap();
	let ctx = dev.new_context(None).unwrap();

    let mut src = ctx.new_static_source().unwrap();
    src.set_looping(false).unwrap();

    let ogg_path = PathBuf::from("Z:\\rust.workspace\\oggs");
    for entry in fs::read_dir(ogg_path).unwrap() {
        let ent = entry.unwrap();

        println!("entry -> {:?}", ent);

        let sound = load_ogg(ent.path()).unwrap();

        let mut buf = ctx.new_buffer().unwrap();
        

        let duration = sound.duration();
        if sound.channels == 1 {
            buf.set_data::<Mono<i16>, _>(sound.data, sound.sample_rate as i32).unwrap();
        } else if sound.channels == 2 {
            buf.set_data::<Stereo<i16>, _>(sound.data, sound.sample_rate as i32).unwrap();
        } else {
            println!("uhh, sound has a weird count of channels -> {:?}", sound.channels);
        }
        
        let buf = Arc::new(buf);
        src.set_buffer(buf).unwrap();
        
        
        src.play().unwrap();
        let play_duration = min(duration + 1.0, 1.0) as u64;
        
        
        println!("Playing for -> {:?}", play_duration);
        std::thread::sleep(std::time::Duration::new(play_duration, 0));
        src.stop().unwrap();
    }

    
    // println!("sound -> {:?}", sound);
    
  
  
}

fn min<T:PartialOrd>(a:T,b:T)->T { if a<b{a}else{b}}

fn max<T:PartialOrd>(a:T,b:T)->T { if a>b{a}else{b}}

struct SinWave {
	len: i32,
	vol: f32,
	cursor: i32,
}

struct SinWaveRenderer<'w>(&'w mut SinWave);


impl SinWave {
	pub fn new(len: i32, vol: f32) -> SinWave {
		SinWave{len: len, vol: vol, cursor: 0}
	}


	pub fn render(&mut self) -> SinWaveRenderer {
		SinWaveRenderer(self)
	}
}


impl<'w> Iterator for SinWaveRenderer<'w> {
	type Item = Mono<i16>;

	fn next(&mut self) -> Option<Mono<i16>> {
		let cursor = self.0.cursor;
		self.0.cursor += 1;
		if self.0.cursor == self.0.len { self.0.cursor = 0 }

		Some(Mono{center: ((cursor as f32 / self.0.len as f32 * 2.0 * std::f32::consts::PI).sin() * self.0.vol * std::i16::MAX as f32) as i16})
	}
}
