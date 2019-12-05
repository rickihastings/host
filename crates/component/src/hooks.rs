use crate::component::{ContextId, ComponentContext, ComponentContextRaw};

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use illicit;

pub struct LocalState {
	context: ComponentContext,
}

impl LocalState {
	#[illicit::from_env(context: &ComponentContext)]
    pub fn new() -> Self {
		let cloned_context = Rc::clone(context);

        Self {
			context: cloned_context
        }
	}

	pub fn set_without_update<T>(&mut self, key: &str, new_value: T)
	where
		T: Clone + Copy + fmt::Display + 'static
	{
		self
			.context
			.borrow_mut()
			.insert_into_state(key, new_value)
	}

	pub fn set<T>(&mut self, key: &str, new_value: T)
	where
		T: Clone + Copy + fmt::Display + 'static
	{
		self.set_without_update(key, new_value);
		self
			.context
			.borrow_mut()
			.application_context
			.borrow_mut()
			.update();
	}

	pub fn get<T>(&mut self, key: &str) -> Option<T>
	where
		T: Clone + Copy + fmt::Display + 'static
	{
		self
			.context
			.borrow_mut()
			.get_from_state(key)
	}
}

pub fn use_state<F, T>(key: &str, data_fn: F) -> (T, LocalState)
where
	T: Clone + Copy + fmt::Display + 'static,
	F: Clone + FnOnce() -> T + 'static
{
	let data = data_fn();
	let mut local_state = LocalState::new();

	if let Some(value) = local_state.get(key) {
		(value, local_state)
	} else {
		local_state.set_without_update(key, data.clone());

		(data, local_state)
	}
}
