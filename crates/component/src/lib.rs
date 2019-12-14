#![feature(associated_type_defaults)]
#![feature(const_fn)]

mod application;
mod component;
mod callsite;
mod reducer;
mod macros;

pub mod prelude {
	use std::rc::Rc;

	pub type ApplicationCtx = Rc<crate::application::ApplicationContext>;

	pub use crate::application::Application;
	pub use crate::component::Component;
	pub use crate::reducer::Reducer;

	pub use crate::html_component;

	pub use virtual_dom_rs::html as html;
	pub use virtual_dom_rs::VirtualNode as VirtualNode;
	pub use virtual_dom_rs::IterableNodes as IterableNodes;
}
