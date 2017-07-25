use super::RectI;
use Color;
use color;

use cgmath::Vector2;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Layer {
    pub frame: RectI,
    pub content: Element,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Element { // "content"
    Text(Text),
    Image(ImageSource),
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct ElementWithSize<F> {
    pub element:Element,
    pub size: Vector2<F>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct ImageSource {
    pub layer: i32,
    pub rect: RectI,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Text {
    pub characters : String,
    pub color: Color,
    pub size: i32,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}

impl Text {
    pub fn new(text:String) -> Text {
        Text {
            characters: text,
            color: color::WHITE,
            size: 10,
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum HorizontalAlignment {
    Left,
    Middle,
    Right,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}