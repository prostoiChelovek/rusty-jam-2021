use rg3d::event::{WindowEvent, Event};
use std::collections::HashMap;
use rg3d::event::VirtualKeyCode;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Forward,
    Backward,
    Left,
    Right
}
 
pub type KeyMap = HashMap<VirtualKeyCode, Action>;
pub type ActionMap<HandlerT> = HashMap<Action, (HandlerT, HandlerT)>;

