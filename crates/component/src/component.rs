use crate::VirtualNode;
use crate::Context;

/// An interface for a React-style Component
pub trait Component {
	fn new() -> Self;

	fn render(&self, context: Context) -> VirtualNode;
}
