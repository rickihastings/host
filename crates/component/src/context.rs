use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

pub trait Listener: Fn() -> () { }
impl<F> Listener for F where F: Fn() -> () { }

// Custom function so we can use Debug on it
pub type ListenerFn = Listener<Output = ()>;

impl fmt::Debug for ListenerFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    { write!(f, "ListenerFn") }
}

pub type Context = Rc<RefCell<RawContext>>;

#[derive(Debug)]
pub struct RawContext {
	listeners: Vec<Box<ListenerFn>>
}

impl RawContext {
    pub fn new() -> Self {
        Self {
			listeners: vec![]
		}
	}

	pub fn subscribe(&mut self, callback: Box<ListenerFn>) {
        self.listeners.push(callback)
	}
	
	pub fn update(&mut self) {
		for callback in self.listeners.iter() {
            callback();
        }
	}
}
