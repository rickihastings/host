use host_component::prelude::*;

#[derive(Copy, Clone)]
pub enum Action {
    IncCount,
}

#[derive(Copy, Clone)]
pub struct DefaultState {
    pub count: u8,
}

#[derive(Copy, Clone)]
pub struct DefaultReducer {
    state: DefaultState,
}

impl Reducer<Action, DefaultState> for DefaultReducer {
    fn new(state: DefaultState) -> Self {
        Self { state }
    }

    fn get_state(&self) -> DefaultState {
        self.state
    }

    fn reducer(&mut self, action: Action) {
        match action {
            Action::IncCount => self.state.count += 1,
        };
    }
}
