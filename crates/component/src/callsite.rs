use fnv::FnvHasher;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::any::TypeId;

/// A value unique to the source location where it is created.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[doc(hidden)]
pub struct ContextId {
	ty: TypeId,
	id: u64
}

impl ContextId {
    #[doc(hidden)]
    pub fn new(ty: TypeId) -> Self {
		let mut callsite = Self {
			ty,
			id: 0
		};
		
		let mut hasher = FnvHasher::default();
        hasher.write_u64(0);
        callsite.hash(&mut hasher);
        callsite.id = hasher.finish();

		callsite
	}
}

impl fmt::Debug for ContextId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:x?}", self.id))
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! callsite {
    () => {{
        struct TotallyRandomAndUniqueStructName;
        $crate::callsite::ContextId::new(std::any::TypeId::of::<TotallyRandomAndUniqueStructName>())
    }};
}
