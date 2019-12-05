use crate::VirtualNode;
use crate::callsite::Callsite;
use crate::application::{ApplicationContext, ApplicationContextRaw};

use illicit;
use fnv::FnvHasher;
use polymap::PolyMap;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;

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
        ComponentContextRaw::run_in_environment(|env| env.borrow_mut().id)
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

pub type ComponentContext = Rc<RefCell<ComponentContextRaw>>;

#[derive(Debug)]
pub struct ComponentContextRaw {
    id: ContextId,
	pub application_context: ApplicationContext,
	state: PolyMap<String>,
}

impl ComponentContextRaw {
    pub fn enter_environment<R>(&self, callsite: Callsite, application_context: ApplicationContext, child: impl FnOnce() -> R) -> R {
        let mut env = Self {
            id: self.id.generate(callsite),
			application_context,
			state: PolyMap::new()
        };

        illicit::child_env!(ComponentContext => Rc::new(RefCell::new(env))).enter(child)
    }

    pub fn run_in_environment<R>(child: impl FnOnce(ComponentContext) -> R) -> R {
        if let Some(env) = illicit::Env::get::<ComponentContext>() {
            child(Rc::clone(&*env))
        } else {
            child(Rc::new(RefCell::new(ComponentContextRaw::default())))
        }
	}

	pub fn insert_into_state<T>(&mut self, key: &str, value: T)
	where
		T: Clone + Copy + fmt::Display + 'static
	{
		let key_string = key.to_string();
		let item = self.state.entry(key_string).or_insert(value);
		*item = value;
	}

	pub fn get_from_state<T>(&mut self, key: &str) -> Option<T>
	where
		T: Clone + Copy + fmt::Display + 'static
	{
		match self.state.get::<String, T>(&key.to_string()) {
			Some(value) => Some(*value),
			None => None
		}
	}
}

impl Default for ComponentContextRaw {
	fn default() -> Self {
		Self {
            id: ContextId(0),
			application_context: Rc::new(RefCell::new(ApplicationContextRaw::new())),
			state: PolyMap::new()
        }
	}
}
