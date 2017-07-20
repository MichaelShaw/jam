use super::RectI;
use Color;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Layer {
    pub frame: RectI,
    pub content: Element,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Element {
    Text(Text),
    Image(ImageSource),
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