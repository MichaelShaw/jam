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

pub fn events<Ev>(last_input: &InputState, input: &InputState, view: View<Ev>) -> Vec<Ev> {
    Vec::new()
}
