
use super::{View, Layer, Element};

use cgmath::Vector2;
use aphid::HashMap;
use InputState;
use ui::{RectI, ZLayer, Widget, Size2, Point2I};

// ok how do we render ...
// we have our own textures




struct UIRenderer {
    pub element_cache : HashMap<(Size2, Element), String>,
}

// doesn't give a shit about anything more than views
impl UIRenderer {
    pub fn render<Ev>(&mut self, view:&View<Ev>) {
        let mut stack :Vec<&View<_>> = Vec::new();

        for v in &view.sub_views {
            stack.push(v);
        }
    }
}

struct WidgetRunner<W> where W : Widget {
    state: W::State,
    view: View<W::Event>,
    last_input: Option<InputState>,
}

impl<W> WidgetRunner<W> where W : Widget {
    pub fn new(initial_state: W::State) -> WidgetRunner<W> {
        let view = W::view(&initial_state);
        WidgetRunner {
            state: initial_state,
            view: view,
            last_input: None,
        }
    }

    pub fn update(&mut self, events: Vec<W::Event>) {
        let mut state_modified = false;
        for ev in events {
            let new_state = W::update(&self.state, &ev);
            if new_state != self.state {
                state_modified = true;
            }
        }
        if state_modified {
            self.view = W::view(&self.state);
        }
    }
}


pub fn events<Ev>(last_input: &InputState, input: &InputState, view: View<Ev>) -> Vec<Ev> {
    Vec::new()
}
