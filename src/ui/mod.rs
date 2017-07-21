pub mod example;

pub mod component;
pub mod render;
pub mod view;
pub mod event;
pub mod layer;

pub use self::view::*;
pub use self::component::*;
pub use self::render::*;
pub use self::event::*;
pub use self::layer::*;

use Color;
use cgmath::{Vector2, BaseNum, vec2};
use image;
use std::fmt::Debug;

pub type ZLayer = i32;
pub type Size2 = Vector2<i32>;
pub type RectI = Rect<i32>;
pub type Point2I = Vector2<i32>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Rect<F> {
    pub min : Vector2<F>,
    pub max : Vector2<F>,
}

impl<F> Rect<F> where F: BaseNum {
    pub fn size(&self) -> Vector2<F> {
        self.max - self.min
    }

    pub fn zeroed(&self) -> Rect<F> {
        Rect {
            min: vec2(F::zero(), F::zero()),
            max: self.max - self.min,
        }
    }

    pub fn new(at:Vector2<F>, size:Vector2<F>) -> Rect<F> {
        Rect {
            min: at,
            max: at + size,
        }
    }

    pub fn offset(&self, v:Vector2<F>) -> Rect<F> {
        Rect {
            min: self.min + v,
            max: self.max + v,
        }
    }

    pub fn with_size(size:Vector2<F>) -> Rect<F> {
        Rect {
           min: Vector2::new(F::zero(), F::zero()),
           max: size,
        }
    }
}

// could be "widget behaviour"
// widget allows &self just for immutable config ... might be a bad idea
pub trait Widget {
    type State : Eq;
    type Event : Debug;
    fn update(&self, st:&Self::State, ev:&Self::Event) -> Self::State;
    fn view(&self, st:&Self::State) -> View<Self::Event>;
}



// ahh shit, we could do blending ...
pub type Bitmap = image::RgbaImage;
pub trait Rasterable {
    fn raster(&self, image: &mut Bitmap, target: Rect<i32>);
}

// events .... mouse down, mouse up, move over (seems reasonable)