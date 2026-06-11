use std::sync::{Condvar, Mutex};
use crate::utils::enums::State;

pub struct ControlState {
    pub state: Mutex<State>,
    pub cv: Condvar
}

impl ControlState {
    pub fn new() -> ControlState {
        todo!()
    }

    pub fn is_cancelled(&self) -> bool {
        let state = self.state.lock().unwrap();
        return *state == State::Cancelled;
    }

    pub fn check_pause(&self) -> bool {
        let state = self.state.lock().unwrap();
        if *state == State::Cancelled {

        }


        return *state == State::Stopped;

    }
}
