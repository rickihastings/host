#![feature(associated_type_defaults)]

mod application;
mod callsite;
mod component;
mod hooks;
mod store;

pub use crate::application::{Application, ApplicationContext};
pub use crate::component::{ContextId, Component, ComponentContext};
pub use crate::hooks::use_state;

pub use virtual_dom_rs::html as html;
pub use virtual_dom_rs::VirtualNode as VirtualNode;
pub use virtual_dom_rs::IterableNodes as IterableNodes;
