use super::View;
use InputState;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum MouseEvent {
    MouseIn,
    MouseOut,
    MouseMove,
    MouseDown,
    MouseUp,
}

pub fn events<Ev>(input: &InputState, last_input: &Option<InputState>, view: &View<Ev>) -> Vec<Ev> {
    println!("generate input events for view");
    Vec::new()
}
