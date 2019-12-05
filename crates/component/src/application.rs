use crate::component::Component;

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Application<T>
where
    T: Component
{
    component: T,
    context: ApplicationContext,
    dom_updater: DomUpdater
}

impl<T> Application<T>
where
    T: Component
{
    pub fn new(root: &str) -> Self {
        let context = Rc::new(RefCell::new(ApplicationContextRaw::new()));
        let component = T::new();

        context.borrow_mut().subscribe(Box::new(|| {
            __host_js.update();
        }));

        // Use `web_sys`'s global `window` function to get a handle on the global window object.
        let window = window().unwrap();
        let document = window.document().unwrap();
        let root_node = document
            .query_selector(&root)
            .expect("cannot find element in document")
            .unwrap();

        let dom_updater = DomUpdater::new_append_to_mount(component.render_to_dom(context.clone()), &root_node);

        Self {
            component,
            context: context,
            dom_updater
        }
    }

    pub fn render(&mut self) {
        self.dom_updater.update(self.component.render_to_dom(self.context.clone()));
    }
}

pub trait Subscriber: Fn() -> () { }

impl<F> Subscriber for F where F: Fn() -> () { }

// Custom function so we can use Debug on it
pub type SubscriberFn = dyn Subscriber<Output = ()>;

impl fmt::Debug for SubscriberFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubscriberFn")
    }
}

#[derive(Debug)]
pub struct ApplicationContextRaw {
    listeners: Vec<Box<SubscriberFn>>
}

impl ApplicationContextRaw {
    pub fn new() -> Self {
        Self {
            listeners: vec![]
        }
    }

    pub fn subscribe(&mut self, callback: Box<SubscriberFn>) {
        self.listeners.push(callback)
    }
    
    pub fn update(&mut self) {
        for callback in self.listeners.iter() {
            callback();
        }
    }
}

pub type ApplicationContext = Rc<RefCell<ApplicationContextRaw>>;

