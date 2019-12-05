use crate::component::{ContextId, ComponentContext};
use crate::store::{insert_into_state, get_from_state};

use std::fmt;

use illicit;

pub struct LocalState {
	id: ContextId,
}

impl LocalState {
	pub fn new() -> Self {
		let id = ContextId::current();

		crate::log!("Hello from {:?}", id);

        Self {
			id
        }
	}

	pub fn set_without_update<T>(&mut self, key: &str, new_value: T)
	where
		T: Clone + Copy + fmt::Display + fmt::Debug + 'static
	{
		insert_into_state(self.id, key, new_value);
	}

	pub fn set<T>(&mut self, key: &str, new_value: T)
	where
		T: Clone + Copy + fmt::Display + fmt::Debug + 'static
	{
		self.set_without_update(key, new_value);

		ComponentContext::run_in_environment(|env| {
			env.application_context.borrow_mut().update();
		});
	}

	pub fn get<T>(&mut self, key: &str) -> Option<T>
	where
		T: Clone + Copy + fmt::Display + fmt::Debug + 'static
	{
		get_from_state(self.id, key)
	}
}

pub fn use_state<F, T>(key: &str, data_fn: F) -> (T, LocalState)
where
	T: Clone + Copy + fmt::Display + fmt::Debug + 'static,
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
