use crate::component::Component;
use crate::state::RcState;

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
	T: Component<RcState>
{
	component: T,
	dom_updater: DomUpdater
}

impl<T> Application<T>
where
	T: Component<RcState>
{
	pub fn new(root: String, state: RcState) -> Self {
		let cloned_state = state.clone();
		let component = T::new(state);

		// Use `web_sys`'s global `window` function to get a handle on the global window object.
		let window = window().unwrap();
		let document = window.document().unwrap();
		let root_node = document
			.query_selector(&root)
			.expect("cannot find element in document")
			.unwrap();

		let dom_updater = DomUpdater::new_append_to_mount(component.mount(), &root_node);
		
		cloned_state.borrow_mut().subscribe(Box::new(|| {
			__host_js.update();
        }));

		Self {
			component,
			dom_updater
		}
	}

	pub fn render(&mut self) {
		self.dom_updater.update(self.component.mount());
	}
}
