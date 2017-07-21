
use super::{View, Layer, Element};

use cgmath::Vector2;
use aphid::HashMap;
use InputState;

use ui::{self, RectI, ZLayer, Widget, Size2, Point2I};

// ok how do we render ...
// we have our own textures





//        for (l, rect, (v_z, l_z)) in view.layer_iter() {
//
//        }

pub struct WidgetRunner<W> where W : Widget {
    widget: W,
    state: W::State,
    view: View<W::Event>,
    last_input: Option<InputState>,
}

impl<W> WidgetRunner<W> where W : Widget {
    pub fn new(widget: W, initial_state: W::State) -> WidgetRunner<W> {
        let view = widget.view(&initial_state);
        WidgetRunner {
            widget,
            state: initial_state,
            view: view,
            last_input: None,
        }
    }

    pub fn run(&mut self, input: InputState, external_events: Vec<W::Event>) {
        let mut all_events = external_events;
        let mut input_events = ui::events(&input, &self.last_input, &self.view);
        all_events.append(&mut input_events);
        self.update(all_events);
        self.last_input = Some(input);
    }

    pub fn update(&mut self, events: Vec<W::Event>) {
        let mut state_modified = false;
        for ev in events {
            println!("applying event -> {:?}", ev);
            let new_state = self.widget.update(&self.state, &ev);
            if new_state != self.state {
                state_modified = true;
            }
        }
        if state_modified {
            self.view = self.widget.view(&self.state);
        }
    }
}

