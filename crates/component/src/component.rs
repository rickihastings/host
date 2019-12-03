extern crate illicit;

use crate::VirtualNode;
use crate::context::Context;
use crate::environment::{Wrapper};

/// An interface for a React-style Component
pub trait Component {
	fn new() -> Self;

	#[doc(hidden)]
	fn render_to_dom(&self, context: Context) -> VirtualNode {
		illicit::child_env!(Context => context).enter(|| {
			self.render()
		})
	}

	fn render(&self) -> VirtualNode;
}
