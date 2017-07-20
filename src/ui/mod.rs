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
use cgmath::{Vector2, BaseNum};
use image;

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
    pub fn offset(&self, v:Vector2<F>) -> Rect<F> {
        Rect {
            min: self.min + v,
            max: self.max + v,
        }
    }

    pub fn with_size(width: F, height: F) -> Rect<F> {
        Rect {
           min: Vector2::new(F::zero(), F::zero()),
           max: Vector2::new(width, height),
        }
    }
}

// could be "widget behaviour"
pub trait Widget {
    type State : Eq;
    type Event;
    fn update(st:&Self::State, ev:&Self::Event) -> Self::State;
    fn view(st:&Self::State) -> View<Self::Event>;
}



// ahh shit, we could do blending ...
pub type Bitmap = image::RgbaImage;
pub trait Rasterable {
    fn raster(&self, image: &mut Bitmap, target: Rect<i32>);
}

// events .... mouse down, mouse up, move over (seems reasonable)