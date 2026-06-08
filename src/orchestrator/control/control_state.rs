use std::sync::{Condvar, Mutex};
use crate::utils::enums::State;

pub struct ControlState {
    pub state: Mutex<State>,
    pub cv: Condvar
}

impl ControlState {
    pub(crate) fn new() -> ControlState {
        todo!()
    }
}
