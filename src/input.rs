extern crate glutin;

use std::collections::HashSet;

pub use glutin::MouseButton;
pub use glutin::VirtualKeyCode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseState {
    pub at: (i32, i32), // could make this optional for off screen? might be a stupid idea.

    pub down: HashSet<glutin::MouseButton>,
    pub pushed: HashSet<glutin::MouseButton>,
    pub released: HashSet<glutin::MouseButton>,

    pub mouse_wheel_delta: i32, // we multiply the float delta by 100 and round it
}

impl MouseState {
    pub fn left_pushed(&self) -> bool {
        self.pushed.contains(&glutin::MouseButton::Left)
    }

    pub fn left_released(&self) -> bool {
        self.released.contains(&glutin::MouseButton::Left)
    }

    pub fn right_pushed(&self) -> bool {
        self.pushed.contains(&glutin::MouseButton::Right)
    }

    pub fn right_released(&self) -> bool {
        self.released.contains(&glutin::MouseButton::Right)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyState {
    pub down: HashSet<glutin::VirtualKeyCode>,
    pub pushed: HashSet<glutin::VirtualKeyCode>,
    pub released: HashSet<glutin::VirtualKeyCode>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputState {
    pub mouse:MouseState,
    pub keys:KeyState,
    pub close:bool,
}

pub fn is_close_event(event: &glutin::Event) -> bool {
    match event {
        &glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | &glutin::Event::Closed => true,
        _ => false,
    }
}

pub fn produce(input:&InputState, events: &Vec<glutin::Event>) -> InputState {
    let mut next_input = input.clone();

    next_input.keys.pushed.clear();
    next_input.keys.released.clear();
    next_input.mouse.pushed.clear();
    next_input.mouse.released.clear();

    next_input.mouse.mouse_wheel_delta = 0;

    for event in events {
        if is_close_event(&event) {
            next_input.close = true;
        }
        match event {
            &glutin::Event::KeyboardInput(element_state, _, Some(key_code)) => 
                match element_state {
                    glutin::ElementState::Pressed => {
                        let was_down = next_input.keys.down.contains(&key_code);
                        next_input.keys.down.insert(key_code);
                        if !was_down {
                            next_input.keys.pushed.insert(key_code);
                        }
                    },
                    glutin::ElementState::Released => {
                        let was_down = next_input.keys.down.contains(&key_code);
                        next_input.keys.down.remove(&key_code);
                        if !was_down {
                            next_input.keys.released.insert(key_code);
                        }
                    }
                },
            &glutin::Event::MouseWheel(glutin::MouseScrollDelta::LineDelta(_, mouse_scroll_delta), _) => {
                next_input.mouse.mouse_wheel_delta += (mouse_scroll_delta * 100.0) as i32;
            },
            &glutin::Event::MouseInput(element_state, mouse_button) =>
                match element_state {
                    glutin::ElementState::Pressed => {
                        let was_down = next_input.mouse.down.contains(&mouse_button);
                        next_input.mouse.down.insert(mouse_button);
                        if !was_down {
                            next_input.mouse.pushed.insert(mouse_button);
                        }
                    },
                    glutin::ElementState::Released => {
                        let was_down = next_input.mouse.down.contains(&mouse_button);
                        next_input.mouse.down.remove(&mouse_button);
                        if was_down {
                            next_input.mouse.released.insert(mouse_button);
                        }
                    },
                },
            &glutin::Event::MouseMoved(x, y) => next_input.mouse.at = (x, y),
            _ => (),
        };
    }

    next_input
}

impl InputState {
    pub fn default() -> InputState {
        InputState {
            mouse: MouseState {
                at: (0, 0),
                down: HashSet::new(),
                pushed: HashSet::new(),
                released: HashSet::new(),
                mouse_wheel_delta: 0,
            },
            keys: KeyState {
                down: HashSet::new(),
                pushed: HashSet::new(),
                released: HashSet::new(),
            },
            close: false,
        }
    }
}
