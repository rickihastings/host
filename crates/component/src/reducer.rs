use polymap::TypeMap;

use std::rc::Rc;
use std::cell::RefCell;

pub trait Reducer<A, S> {
	fn new(state: S) -> Self;

	fn get_state(&self) -> S;

	fn reducer(&mut self, action: A);
}

pub struct ReducerTree {
    reducers: TypeMap,
}

impl ReducerTree {
	pub fn new() -> Self {
		Self {
			reducers: TypeMap::new(),
		}
	}

	pub fn insert<R, A, S>(&mut self, reducer: Rc<RefCell<R>>)
	where
		R: Reducer<A, S> + 'static,
		A: Copy + Sized + 'static,
		S: Sized + 'static,
	{
		self.reducers.entry::<Rc<RefCell<R>>>().or_insert(reducer);
	}

	pub fn dispatch<R, A, S>(&mut self, action: A)
	where
		R: Reducer<A, S> + 'static,
		A: Copy + Sized + 'static,
		S: Sized + 'static,
	{
		if let Some(value) = self.reducers.get::<Rc<RefCell<R>>>() {
			value.borrow_mut().reducer(action);
		}
	}

	pub fn get_state<R, A, S>(&self) -> Option<S>
    where
        R: Reducer<A, S> + 'static,
        A: Copy + Sized + 'static,
		S: Sized + 'static,
	{
		if let Some(value) = self.reducers.get::<Rc<RefCell<R>>>() {
			Some(value.borrow().get_state())
		} else {
			None
		}
	}
}
