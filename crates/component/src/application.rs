use crate::component::Component;
use crate::context::{RawContext, Context};

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{window};
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
	context: Context,
	dom_updater: DomUpdater
}

impl<T> Application<T>
where
	T: Component
{
	pub fn new(root: String) -> Self {
		let context = Rc::new(RefCell::new(RawContext::new()));
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
