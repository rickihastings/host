use std::cell::RefCell;
use std::rc::Rc;

pub type RcState = Rc<RefCell<State>>;

pub struct State {
	listeners: Vec<Box<Fn() -> ()>>
}

impl State {
    pub fn new() -> Self {
        Self {
			listeners: vec![]
		}
	}

	pub fn subscribe(&mut self, callback: Box<Fn() -> ()>) {
        self.listeners.push(callback)
	}
	
	pub fn update(&mut self) {
		for callback in self.listeners.iter() {
            callback();
        }
	}
}
