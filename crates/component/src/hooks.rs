use crate::store::{insert_into_state, get_from_state};
use crate::application::{CONTEXT_CONTAINER, ApplicationContext};
use crate::callsite::ContextId;
use crate::component::Component;

use std::fmt;
use std::sync::Mutex;

pub struct LocalState {
    id: ContextId,
}

impl LocalState {
    pub fn new(id: ContextId) -> Self {
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

        let application_context = CONTEXT_CONTAINER.get::<Mutex<ApplicationContext>>();
        if let Some(mut context) = application_context.lock().ok() {
            context.update();
        }
    }

    pub fn get<T>(&mut self, key: &str) -> Option<T>
    where
        T: Clone + Copy + fmt::Display + fmt::Debug + 'static
    {
        get_from_state(self.id, key)
    }
}

pub fn use_state<C, F, T>(component: &C, key: &str, data_fn: F) -> (T, LocalState)
where
    C: Component,
    T: Clone + Copy + fmt::Display + fmt::Debug + 'static,
    F: Clone + FnOnce() -> T + 'static
{
    let data = data_fn();
    let mut local_state = LocalState::new(component.get_component_id());

    if let Some(value) = local_state.get(key) {
        (value, local_state)
    } else {
        local_state.set_without_update(key, data.clone());

        (data, local_state)
    }
}
