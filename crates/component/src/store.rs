use crate::component::ContextId;

use std::fmt;
use std::cell::RefCell;
use std::collections::HashMap;

use polymap::PolyMap;

thread_local! {
    static STORE: RefCell<Store> = RefCell::new(Store::new());
}

pub type Store = HashMap<ContextId, PolyMap<String>>;

pub fn insert_into_state<T>(id: ContextId, key: &str, value: T)
where
	T: Clone + Copy + fmt::Display + fmt::Debug + 'static
{
	STORE.with(|store_refcell| {
		let key_string = key.to_string();
		let mut store = store_refcell.borrow_mut();
		
		let id_map = store
			.entry(id)
			.or_insert(PolyMap::new());

		let item = id_map
			.entry(key_string)
			.or_insert(value);

		*item = value;
	});
}

pub fn get_from_state<T>(id: ContextId, key: &str) -> Option<T>
where
	T: Clone + Copy + fmt::Display + 'static
{
	STORE.with(|store_refcell| {
		if let Some(id_map) = store_refcell.borrow_mut().get(&id) {
			match id_map.get::<String, T>(&key.to_string()) {
				Some(value) => Some(*value),
				None => None
			}
		} else {
			None
		}
	})
}
