

use super::{MouseEvent, View, Rect, ImageSource};

pub fn button<Ev, F>(frame: Rect<i32>,
                     text: String,
                     icon: ImageSource,
                     on_click: F) -> View<Ev> where F : 'static + Fn() -> Option<Ev>  {
    let nf = move |me| {
        if me == MouseEvent::MouseUp {
            on_click()
        } else {
            None
        }
    };

    View {
        frame,
        on_event: Some(Box::new(nf)),
        layers: Vec::new(),
        sub_views : Vec::new(),
    }
}

pub enum MyEvent {
    Click,
}

pub fn button_me() {
    let icon = ImageSource {
        layer: 0,
        rect: Rect::with_size(100, 100),
    };
    let x : View<MyEvent> = button(Rect::with_size(100, 100), "sup".into(), icon, || { Some(MyEvent::Click) });
}