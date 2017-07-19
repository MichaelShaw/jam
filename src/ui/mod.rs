pub mod example;

use cgmath::Vector2;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Rect<F> {
    pub min : Vector2<F>,
    pub max : Vector2<F>,
}


// how do we do layout!?
// views need to layout their elements ....
// and subviews ....

// to render something
// render :: BaseImage -> Rect -> Element -> BaseImage //

pub struct View<Ev> {
    pub content: Vec<Element>,
    pub on_event: Box<Fn(MouseEvent) -> Option<Ev>>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum MouseEvent {
    MouseIn,
    MouseOut,
    MouseMove,
    MouseDown,
    MouseUp,
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

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Text {
    pub characters : String,
    pub size: i32,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct ImageSource {
    pub layer: i32,
    pub rect: Rect<i32>,
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Element {
    Text(Text),
    Image(ImageSource),
}

// could be "widget behaviour"
pub trait Widget<St, Ev> where St: Eq {
    fn update(st:St, ev:&Ev) -> St;
    fn view(st:&St) -> View<Ev>;
}

// events .... mouse down, mouse up, move over (seems reasonable)