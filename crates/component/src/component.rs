use crate::prelude::VirtualNode;
use crate::callsite::ContextId;
use crate::application::ApplicationContext;

use std::collections::HashMap;
use std::rc::Rc;

/// An interface for a React-style Component
pub trait Component {
    fn id(&self) -> ContextId {
        crate::callsite!()
    }

    fn prepare_render(&self, context: Rc<ApplicationContext>) -> VirtualNode {
        illicit::child_env!(Rc<ApplicationContext> => context).enter(|| {
            self.render()
        })
    }

    fn render(&self) -> VirtualNode;
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
