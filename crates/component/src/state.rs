use crate::LocalContext;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::collections::HashMap;

use comp_state::topo::Id;
use serde::Serialize;
use serde_json::{value::Value, json};

// thread storage
thread_local! {
	pub static STORE: RefCell<Store> = RefCell::new(Store::new());
}

pub struct Store {
	map: HashMap::<Id, HashMap<String, Value>>,
}

impl Store {
	pub fn new() -> Self {
		Self {
			map: HashMap::new()
		}
	}

	pub fn insert_to_store<T>(&mut self, id: Id, key: &str, value: T)
	where
		T: 'static + Clone + serde::Serialize
	{
		if let Some(current_value) = self.map.get_mut(&id) {
			// Id exists in hashmap, lets add our new state to it.
			// *current_value = String::from("0");
		} else {
			// Id doesn't exist so we need to add it
			let mut state_map = HashMap::<String, Value>::new();

			state_map.insert(key.to_string(), json!(value));

			self.map.insert(id, state_map);
		}
	}

	pub fn update_in_store<T>(&mut self, id: Id, key: &str, value: T)
	where
		T: 'static + Clone + serde::Serialize
	{
		
	}
}

// local state
pub struct LocalState<T> {
	pub id: Id,
	_phantom_data: PhantomData<T>,
}

impl<T> LocalState<T>
where
    T: 'static + Clone + serde::Serialize
{
    pub fn new(id: Id) -> Self {
        Self {
            id,
            _phantom_data: PhantomData,
        }
	}

	pub fn set(&self, new_value: T) {

	}
}

pub fn use_state<T, F>(key: &str, data_fn: F) -> (T, LocalState<T>)
where
	T: 'static + Clone + serde::Serialize,
	F: 'static + Clone + FnOnce() -> T
{
	let current_context = 
	let current_id = Id::current();
	let data = data_fn();

	set_state_with_id(current_id, key, data.clone());

	(data, LocalState::new(current_id))
}

fn set_state_with_id<T>(id: Id, key: &str, value: T)
where
	T: 'static + Clone + serde::Serialize
{
	STORE.with(|store_rc| {
		store_rc.borrow_mut().insert_to_store(id, key, value);
	});
}

fn update_state_with_id<T>(id: Id, key: &str, value: T)
where
	T: 'static + Clone + serde::Serialize
{
	STORE.with(|store_rc| {
		store_rc.borrow_mut().update_in_store(id, key, value);
	});
}
