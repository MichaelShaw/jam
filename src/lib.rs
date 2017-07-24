#![crate_name="jam"]
#![allow(dead_code)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_device_gl;

extern crate time;

extern crate cgmath;
extern crate image;

extern crate notify;
extern crate rusttype;
#[macro_use]
extern crate aphid;


pub mod render;
pub mod camera;
pub mod input;
pub mod geometry;
pub mod spring;
pub mod color;
pub mod font;
pub mod dimensions;
pub mod ui;

pub use font::*;
pub use camera::*;
pub use color::*;
pub use dimensions::*;
pub use font::*;
pub use geometry::*;
pub use input::*;
pub use spring::*;

use std::path::PathBuf;
use std::path::Path;

use std::fs::File;


use std::io;
use std::io::Read;


pub type JamResult<T> = Result<T, JamError>;

#[derive(Debug)]
pub enum JamError {
    IO(io::Error),
    FileDoesntExist(PathBuf),
    PipelineError(gfx::PipelineStateError<String>),
    CombinedGFXError(gfx::CombinedError),
//    ProgramLoadError(glium::ProgramCreationError),
//    TextureLoadError(glium::texture::TextureCreationError),
    FontLoadError(font::FontLoadError),
    ImageError(image::ImageError),
//    WindowCreationError(glium::backend::glutin::DisplayCreationError),
//    SwapBufferError(glium::SwapBuffersError),
    MustLoadTextureBeforeFont,
    NoFiles,
    MismatchingDimensions, // path buf, expectation
    RenderingPipelineIncomplete,
}

impl From<font::FontLoadError> for JamError {
    fn from(err: font::FontLoadError) -> Self {
        JamError::FontLoadError(err)
    }
}

impl From<image::ImageError> for JamError {
    fn from(err: image::ImageError) -> Self {
        JamError::ImageError(err)
    }
}

impl From<io::Error> for JamError {
    fn from(val: io::Error) -> JamError {
        JamError::IO(val)
    }
}


pub fn clamp<T : PartialOrd>(n:T, min:T, max:T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

pub fn vec2i(x:i32, y:i32) -> Vec2i {
    Vec2i::new(x, y)
}

pub fn lerp(a:Vec3, b:Vec3, alpha:f64) -> Vec3 {
    b * alpha + a * (1.0 - alpha)
}

pub type Vec2 = cgmath::Vector2<f64>;
pub type Vec2f = cgmath::Vector2<f32>;
pub type Vec2i = cgmath::Vector2<i32>;
pub type Vec2Size = cgmath::Vector2<usize>;
pub type Vec3i = cgmath::Vector3<i32>;

pub type Vec3 = cgmath::Vector3<f64>;
pub type Vec3f = cgmath::Vector3<f32>;

pub type Vec4 = cgmath::Vector4<f64>;

pub type Mat3 = cgmath::Matrix3<f64>;
pub type Mat4 = cgmath::Matrix4<f64>;

pub fn round_down(f:f64) -> i32 {
    if f < 0.0 {
        f as i32 - 1
    } else {
        f as i32
    }
}

pub fn round_down_v3(v:Vec3) -> Vec3i {
    Vec3i::new(round_down(v.x), round_down(v.y), round_down(v.z))
}

pub fn load_file_contents(path:&Path) -> io::Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buffer : Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buffer));
    Ok(buffer)
}
