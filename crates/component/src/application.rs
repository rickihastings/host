use crate::component::{Component, ComponentContext};
use crate::state::with_app_mut;

use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::window;
use virtual_dom_rs::DomUpdater;

#[wasm_bindgen]
extern "C" {
    pub type HostJS;

    pub static __host_js: HostJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &HostJS);
}

pub struct Application {
    component: Box<dyn Component>,
    dom_updater: DomUpdater
}

impl Application {
    pub fn new(root: &str, boxed_component: Box<dyn Component>) -> Self {
        let component_context = ComponentContext::new(boxed_component.id(), 0, boxed_component);

        with_app_mut(|context| {
            context.insert_listener(Box::new(|| {
                __host_js.update();
            }));
        });

        // Use `web_sys`'s global `window` function to get a handle on the global window object.
        let window = window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .query_selector(&root)
            .expect("cannot find element in document")
            .unwrap();

        let dom_updater = DomUpdater::new_append_to_mount(
            component_context.component.prepare_render(),
            &root_node
        );

        Self {
            component: component_context.component,
            dom_updater
        }
    }

    pub fn render(&mut self) {
        with_app_mut(|context| {
            context.increment_render_count();
        });

        let new_dom = self.component.prepare_render();

        // Check for any components which will be unmounted.
        // if let Some(mut component_tree) = COMPONENT_TREE.lock().ok() {
        //     // component_tree.unmount_components(render_count);
        // }

        self.dom_updater.update(new_dom);
    }
}

pub struct ApplicationContext {
    listeners: Vec<Box<dyn Fn() + Send + Sync>>,
    render_count: u8,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
            render_count: 0,
        }
    }

    pub fn increment_render_count(&mut self) {
        self.render_count += 1;
    }

    pub fn insert_listener(&mut self, on_render: Box<dyn Fn() + Send + Sync>) {
        self.listeners.push(on_render);
    }
    
    pub fn update(&mut self) {
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn get_render_count(&self) -> u8 {
        self.render_count
    }
}

