use crate::component::{Component, ComponentContext, ComponentTree};
use crate::reducer::{Reducer, ReducerTree};

use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::window;
use virtual_dom_rs::DomUpdater;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

type MutableReducerTree = Rc<RefCell<ReducerTree>>;

#[wasm_bindgen]
extern "C" {
    pub type HostJS;

    pub static __host_js: HostJS;

    #[wasm_bindgen(method)]
    pub fn update(this: &HostJS);
}

pub struct Application {
    root: String,
    dom_updater: Option<DomUpdater>,
    context: Rc<ApplicationContext>,
    component_tree: ComponentTree,
    reducer_tree: MutableReducerTree,
}

impl Application {
    pub fn new(root: &str, boxed_component: Box<dyn Component>) -> Self {
        let component_context = ComponentContext::new(boxed_component.id(), boxed_component);
        let mut component_tree = ComponentTree::new();
        let reducer_tree = Rc::new(RefCell::new(ReducerTree::new()));

        let mut application_context = ApplicationContext::new(Rc::clone(&reducer_tree));

        application_context.insert_listener(Box::new(|| {
            __host_js.update();
        }));

        component_tree.insert(component_context);

        Self {
            root: String::from(root),
            dom_updater: None,
            context: Rc::new(application_context),
            component_tree: component_tree,
            reducer_tree: Rc::clone(&reducer_tree),
        }
    }

    pub fn inject_store<R, A, S>(&mut self, reducer: R)
    where
        R: Reducer<A, S> + 'static,
        A: Copy + Sized + 'static,
        S: Sized + 'static,
    {
        self.reducer_tree
            .borrow_mut()
            .insert(Rc::new(RefCell::new(reducer)));
    }

    pub fn mount(&mut self) {
        // Find the first component
        if let Some(component_context) = self.component_tree.get_first_component() {
            // Use `web_sys`'s global `window` function to get a handle on the global window object.
            let window = window().unwrap();
            let document = window.document().unwrap();
            let root_node = document
                .query_selector(&self.root)
                .expect("cannot find element in document")
                .unwrap();

            let dom_updater = DomUpdater::new_append_to_mount(
                component_context.component.prepare_render(Rc::clone(&self.context)),
                &root_node
            );

            self.dom_updater = Some(dom_updater);
        }
    }

    pub fn render(&mut self) {
        if let Some(dom_updater) = self.dom_updater.as_mut() {
            if let Some(component_context) = self.component_tree.get_first_component() {
                dom_updater.update(
                    component_context.component.prepare_render(Rc::clone(&self.context))
                );
            }
        }
    }
}

pub trait Listener: Fn() -> () { }
impl<F> Listener for F where F: Fn() -> () { }

type BoxedListener = Box<dyn Listener + Send + Sync>;

impl fmt::Debug for BoxedListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("BoxedListener"))
    }
}

#[derive(Debug)]
pub struct ApplicationContext {
    listeners: Vec<BoxedListener>,
    reducer_tree: MutableReducerTree,
}

impl ApplicationContext {
    pub fn new(reducer_tree: MutableReducerTree) -> Self {
        Self {
            listeners: Vec::new(),
            reducer_tree,
        }
    }

    pub fn insert_listener(&mut self, on_render: BoxedListener) {
        self.listeners.push(on_render);
    }

    pub fn dispatch<R, A, S>(&self, action: A)
    where
        R: Reducer<A, S> + 'static,
        A: Copy + Sized + 'static,
        S: Sized + 'static,
    {
        self.reducer_tree
            .borrow_mut()
            .dispatch::<R, A, S>(action);

        self.update();
    }

    pub fn get_state<R, A, S>(&self) -> Option<S>
    where
        R: Reducer<A, S> + 'static,
        A: Copy + Sized + 'static,
        S: Sized + 'static,
    {
        self.reducer_tree
            .borrow_mut()
            .get_state::<R, A, S>()
    }
    
    pub fn update(&self) {
        for callback in self.listeners.iter() {
            callback();
        }
    }
}

