extern crate illicit;

use crate::context::{RawContext, Context};

use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

// Mostly based off topo

#[derive(Copy, Clone)]
pub struct Id(u64);

impl Id {
    /// Returns the `Id` for the current scope in the call topology.
    pub fn current() -> Self {
        Environment::run_in_environment(|env| env.id)
    }

    fn generate(self, callsite: Callsite) -> Self {
        let mut hasher = FnvHasher::default();
        hasher.write_u64(self.0);
        callsite.hash(&mut hasher);
        Id(hasher.finish())
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x?}", self.0))
    }
}

#[derive(Debug)]
pub struct Environment {
    id: Id,
    pub context: Context
}

impl Environment {
    pub fn enter_environment<R>(&self, callsite: Callsite, context: Context, child: impl FnOnce() -> R) -> R {
        let env = Self {
            id: self.id.generate(callsite),
            context,
        };

        illicit::child_env!(Environment => env).enter(child)
    }

    pub fn run_in_environment<R>(child: impl FnOnce(&Environment) -> R) -> R {
        if let Some(env) = illicit::Env::get::<Self>() {
            child(&*env)
        } else {
            child(&Environment::default())
        }
    }
}

impl Default for Environment {
	fn default() -> Self {
		Self {
            id: Id(0),
            context: std::rc::Rc::new(std::cell::RefCell::new(RawContext::new()))
        }
	}
}

/// A value unique to the source location where it is created.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Callsite {
    ty: std::any::TypeId,
}

impl Callsite {
    #[doc(hidden)]
    pub fn new(ty: std::any::TypeId) -> Self {
        Self { ty }
    }
}

#[macro_export]
macro_rules! callsite {
    () => {{
        struct TotallyRandomAndUniqueStructName;
        $crate::environment::Callsite::new(std::any::TypeId::of::<TotallyRandomAndUniqueStructName>())
    }};
}

#[macro_export]
macro_rules! call {
    ($context:expr, $($input:tt)*) => {{
        let callsite = $crate::callsite!();
        $crate::environment::Environment::run_in_environment(|current| {
            current.enter_environment(callsite, $context, $($input)*)
        })
    }};
}
