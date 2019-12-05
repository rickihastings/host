use std::any::TypeId;

// Influenced by Topo

/// A value unique to the source location where it is created.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Callsite {
    ty: TypeId,
}

impl Callsite {
    #[doc(hidden)]
    pub fn new(ty: TypeId) -> Self {
        Self {
			ty
		}
    }
}

#[macro_export]
macro_rules! callsite {
    () => {{
        struct TotallyRandomAndUniqueStructName;
        $crate::callsite::Callsite::new(std::any::TypeId::of::<TotallyRandomAndUniqueStructName>())
    }};
}

#[macro_export]
macro_rules! call_in_context {
    ($context:expr, $($input:tt)*) => {{
        let callsite = $crate::callsite!();
        $crate::component::ComponentContextRaw::run_in_environment(|current| {
            current.borrow_mut().enter_environment(callsite, $context, $($input)*)
        })
    }};
}