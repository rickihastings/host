use crate::VirtualNode;
use crate::callsite::Callsite;
use crate::application::{ApplicationContext, ApplicationContextRaw};

use illicit;
use fnv::FnvHasher;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

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
    fn render_to_dom(&self, context: ApplicationContext) -> VirtualNode {
        crate::call_in_context!(context, || {
            self.render()
        })
    }

    fn render(&self) -> VirtualNode;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ContextId(u64);

impl ContextId {
    /// Returns the `Id` for the current scope in the call topology.
    pub fn current() -> Self {
        ComponentContext::run_in_environment(|env| env.id)
    }

    fn generate(self, callsite: Callsite) -> Self {
        let mut hasher = FnvHasher::default();
        hasher.write_u64(self.0);
        callsite.hash(&mut hasher);
        ContextId(hasher.finish())
    }
}

impl fmt::Debug for ContextId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:x?}", self.0))
    }
}

#[derive(Debug)]
pub struct ComponentContext {
    pub id: ContextId,
    pub application_context: ApplicationContext,
}

impl ComponentContext {
    pub fn enter_environment<R>(&self, callsite: Callsite, application_context: ApplicationContext, child: impl FnOnce() -> R) -> R {
        let env = Self {
            id: self.id.generate(callsite),
            application_context,
        };

        illicit::child_env!(ComponentContext => env).enter(child)
    }

    pub fn run_in_environment<R>(child: impl FnOnce(&ComponentContext) -> R) -> R {
        if let Some(env) = illicit::Env::get::<Self>() {
            child(&*env)
        } else {
            child(&ComponentContext::default())
        }
    }
}

impl Default for ComponentContext {
    fn default() -> Self {
        Self {
            id: ContextId(0),
            application_context: Rc::new(RefCell::new(ApplicationContextRaw::new())),
        }
    }
}
