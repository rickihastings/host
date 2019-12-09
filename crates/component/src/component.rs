use crate::VirtualNode;
use crate::callsite::ContextId;

use std::collections::HashMap;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! component {
    ( $c:ident { $($opt:expr),* } ) => {
        let __component = $c::new($($opt)*);
        let __boxed_component = Box::new(__component);
        let __component_context = $crate::ComponentContext::new(__boxed_component.id(), 0, __boxed_component);

        $crate::with_comp_tree_mut(|context| {
            context.insert(__component_context);
        });

        __component.prepare_render()
    }
}

/// An interface for a React-style Component
pub trait Component: Send + Sync {
    fn id(&self) -> ContextId {
        crate::callsite!()
    }

    #[doc(hidden)]
    fn prepare_render(&self) -> VirtualNode {
        // let render_count = get_from_app(|context| {
        //     context.get_render_count()
        // }, 0);

        self.render()
    }

    fn will_unmount(&self) {}

    fn render(&self) -> VirtualNode;
}

pub struct ComponentContext {
    id: ContextId,
    render_count: u8,
    pub component: Box<dyn Component>
}

impl ComponentContext {
    pub fn new(id: ContextId, render_count: u8, component: Box<dyn Component>) -> Self {
        Self {
            id,
            render_count,
            component
        }
    }

    pub fn get_id(&self) -> ContextId {
        self.id
    }

    pub fn inc_render_count(&mut self) {
        self.render_count += 1;
    }

    pub fn get_render_count(&self) -> u8 {
        self.render_count
    }
}

pub struct ComponentTree {
    components: HashMap<ContextId, ComponentContext>,
}

impl ComponentTree {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, component_context: ComponentContext) {
        self.components.entry(component_context.get_id()).or_insert(component_context);
    }

    pub fn get_mut<C>(&mut self, id: ContextId) -> Option<&mut C> {
        if let Some(component_context) = self.components.get_mut(&id) {
            Some(&mut component_context.component);
        }

        None
    }
}
