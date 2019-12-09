#![feature(associated_type_defaults)]
#![feature(const_fn)]

#[macro_use]
extern crate lazy_static;

mod application;
mod component;
mod state;
mod callsite;
mod store;

pub use crate::application::{Application, ApplicationContext};
pub use crate::component::{Component, ComponentContext};
pub use crate::state::{with_comp_tree_mut, get_component_mut};

pub use virtual_dom_rs::html as html;
pub use virtual_dom_rs::VirtualNode as VirtualNode;
pub use virtual_dom_rs::IterableNodes as IterableNodes;
