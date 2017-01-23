use lewton;

use lewton::VorbisError;
use lewton::inside_ogg::OggStreamReader;

use std::fs::File;
use std::path::{Path,PathBuf};

use ogg;

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
