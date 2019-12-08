use crate::component::Component;

use std::sync::Mutex;

use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::window;
use virtual_dom_rs::DomUpdater;
use state::Container;

pub static CONTEXT_CONTAINER: Container = Container::new();

#[wasm_bindgen]
extern "C" {
    pub type HostJS;

    pub static __host_js: HostJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &HostJS);
}

pub struct Application<T>
where
    T: Component
{
    component: T,
    dom_updater: DomUpdater
}

impl<T> Application<T>
where
    T: Component
{
    pub fn new(root: &str) -> Self {
        let application_context = Mutex::new(ApplicationContext::new(Box::new(|| {
            __host_js.update();
        })));

        CONTEXT_CONTAINER.set(application_context);

        let component = T::new();

        // Use `web_sys`'s global `window` function to get a handle on the global window object.
        let window = window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .query_selector(&root)
            .expect("cannot find element in document")
            .unwrap();

        let dom_updater = DomUpdater::new_append_to_mount(component.render(), &root_node);

        Self {
            component,
            dom_updater
        }
    }

    pub fn render(&mut self) {
        self.dom_updater.update(self.component.render());
    }
}

pub struct ApplicationContext {
    listeners: Vec<Box<Fn() + Send + Sync>>,
}

impl ApplicationContext {
    pub fn new(on_render: Box<Fn() + Send + Sync>) -> Self {
        Self {
            listeners: vec![on_render]
        }
    }
    
    pub fn update(&mut self) {
        for callback in self.listeners.iter() {
            callback();
        }
    }
}

