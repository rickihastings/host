use crate::VirtualNode;
use crate::callsite::ContextId;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// An interface for a React-style Component
pub trait Component {
    fn new() -> Self;

    #[doc(hidden)]
    fn get_component_id(&self) -> ContextId {
        crate::callsite!()
    }

    fn render(&self) -> VirtualNode;
}
