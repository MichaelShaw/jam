
use super::{View, Layer, Element};

use cgmath::Vector2;
use aphid::HashMap;
use {InputState, Dimensions};


use ui::{self, RectI, ZLayer, Widget, Size2, Point2I};

pub struct WidgetRunner<W> where W : Widget {
    widget: W,
    pub state: W::State,
    view: View<W::Event>,
    last_input: Option<InputState>,
    last_dimensions: Dimensions,
}

impl<W> WidgetRunner<W> where W : Widget {
    pub fn view(&self) -> &View<W::Event> {
        &self.view
    }

    pub fn new(widget: W, initial_state: W::State, dimensions:Dimensions) -> WidgetRunner<W> {
        let view = widget.view(&initial_state, dimensions);
        WidgetRunner {
            widget,
            state: initial_state,
            view: view,
            last_input: None,
            last_dimensions: dimensions,
        }
    }

    pub fn run(&mut self, input: InputState, external_events: Vec<W::Event>, dimensions:Dimensions) {
        let mut all_events = external_events;
        let mut input_events = ui::events(&input, &self.last_input, &self.view);
        all_events.append(&mut input_events);
        self.update(all_events, dimensions);
        self.last_input = Some(input);
    }

    pub fn update(&mut self, events: Vec<W::Event>, dimensions:Dimensions) {
        let mut state_modified = false;
        for ev in events {
//            println!("applying event -> {:?}", ev);
            let new_state = self.widget.update(&self.state, &ev);
            if new_state != self.state {
//                println!("state modified!");
                state_modified = true;
                self.state = new_state;
            }
        }
        if state_modified || dimensions != self.last_dimensions {
//            println!("regenerating view");
            self.view = self.widget.view(&self.state, dimensions);
            self.last_dimensions = dimensions;
        }
    }
}

