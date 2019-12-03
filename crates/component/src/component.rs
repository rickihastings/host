extern crate illicit;

use crate::VirtualNode;
use crate::context::Context;


/// An interface for a React-style Component
pub trait Component {
	fn new() -> Self;

	#[doc(hidden)]
	fn render_to_dom(&self, context: Context) -> VirtualNode {
		crate::call!(context, || {
			self.render()
		})
	}

	fn render(&self) -> VirtualNode;
}
