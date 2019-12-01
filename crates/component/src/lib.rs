mod component;
mod application;
mod state;

use comp_state::*;

pub use crate::application::Application;
pub use crate::component::Component;
pub use crate::state::RcState;
pub use crate::state::State;

pub use virtual_dom_rs::html as html;
pub use virtual_dom_rs::VirtualNode as VirtualNode;
pub use virtual_dom_rs::IterableNodes as IterableNodes;

pub use comp_state::topo as topo;

use std::marker::PhantomData;

///  Accessor struct that provides access to getting and setting the
///  state of the stored type
pub struct StateAccess<T> {
    pub id: topo::Id,
    state: RcState,
    _phantom_data: PhantomData<T>,
}

impl<T> StateAccess<T>
where
    T: 'static + Clone,
{
    pub fn new(id: topo::Id, global_state: RcState) -> StateAccess<T> {
        StateAccess {
            id,
            state: global_state,
            _phantom_data: PhantomData,
        }
    }

    // stores a value of type T in a backing Store
    pub fn set(&self, value: T) {
        set_state_with_topo_id(value, self.id);

        self.state.borrow_mut().update();
    }

    /// updates the stored state in place
    /// using the provided function
    pub fn update<F: FnOnce(&mut T) -> ()>(&self, func: F) {
        let item = &mut self.get().unwrap();
        func(item);
        self.set(item.clone());
    }

    /// returns a option clone of the stored state.
    pub fn get(&self) -> Option<T> {
        get_state_with_topo_id::<T>(self.id)
    }

    /// returns a clone of the stored state panics if not stored.
    pub fn hard_get(&self) -> T {
        get_state_with_topo_id::<T>(self.id).unwrap()
    }
}

pub fn use_state<T, F>(global_state: RcState, data_fn: F) -> (T, StateAccess<T>)
where
    T: 'static + Clone,
    F: FnOnce() -> T
{
    let current_id = topo::Id::current();

    // returns a clone of the curent stored type. If the type has not been stored before
    // set it with the closure passed to use_state.
    if let Some(stored_data) = clone_state::<T>() {
        (stored_data, StateAccess::new(current_id, global_state))
    } else {
        let data = data_fn();
        set_state_with_topo_id::<T>(data.clone(), current_id);
        (data, StateAccess::new(current_id, global_state))
    }
}
