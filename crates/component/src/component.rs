use crate::prelude::VirtualNode;
use crate::callsite::ContextId;
use crate::application::ApplicationContext;

use std::collections::HashMap;
use std::rc::Rc;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! component {
    ( $c:ident { $($opt:expr),* } ) => {
        let mut __component = $c::new($($opt)*);
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
    fn prepare_render(&mut self, ctx: Rc<ApplicationContext>) -> VirtualNode {
        self.render(ctx)
    }

    fn render(&mut self, ctx: Rc<ApplicationContext>) -> VirtualNode;
}

pub struct ComponentContext {
    id: ContextId,
    pub component: Box<dyn Component>
}

impl ComponentContext {
    pub fn new(id: ContextId, component: Box<dyn Component>) -> Self {
        Self {
            id,
            component
        }
    }

    pub fn get_id(&self) -> ContextId {
        self.id
    }
}

pub struct ComponentTree {
    components: HashMap<ContextId, ComponentContext>,
    component_id_list: Vec<ContextId>,
}

impl ComponentTree {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            component_id_list: Vec::new(),
        }
    }

    pub fn insert(&mut self, component_context: ComponentContext) {
        let id = component_context.get_id();

        if let None = self.component_id_list.iter().find(|item| *item == &id) {
            self.component_id_list
                .push(id);
        }
        
        self.components
            .entry(id)
            .or_insert(component_context);
    }

    pub fn get_first_component(&mut self) -> Option<&mut ComponentContext> {
        if let Some(id) = self.component_id_list.first() {
            self.components.get_mut(&id)
        } else {
            None
        }
    }
}
