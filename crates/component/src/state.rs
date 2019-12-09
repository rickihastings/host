use crate::application::ApplicationContext;
use crate::callsite::ContextId;
use crate::component::{Component, ComponentTree};

use std::sync::Mutex;

lazy_static! {
    static ref APP_CONTEXT: Mutex<ApplicationContext> = Mutex::new(ApplicationContext::new());
	static ref COMPONENT_TREE: Mutex<ComponentTree> = Mutex::new(ComponentTree::new());
}

pub fn with_app_mut<F>(with: F)
where
	F: FnOnce(&mut ApplicationContext) -> ()
{
	if let Some(mut context) = APP_CONTEXT.lock().ok() {
		with(&mut context);
	}
}

pub fn get_from_app<F, V>(with: F, default_value: V) -> V
where
	F: FnOnce(&ApplicationContext) -> V
{
	if let Some(context) = APP_CONTEXT.lock().ok() {
		with(&context)
	} else {
		default_value
	}
}

pub fn with_comp_tree_mut<F>(with: F)
where
	F: FnOnce(&mut ComponentTree) -> ()
{
	if let Some(mut context) = COMPONENT_TREE.lock().ok() {
		with(&mut context);
	}
}

pub fn get_component_mut<C, F: Fn(&mut C) -> ()>(id: ContextId, with: F)
where
	C: Component
{
	if let Some(mut context) = COMPONENT_TREE.lock().ok() {
		if let Some(mut component) = context.get_mut(id) {
			with(&mut component);
		}
	}
}
