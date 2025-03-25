use std::{fmt, hash::Hash, hash::Hasher};
use crate::util::DynLabel;

pub trait PageLabel : Send + DynLabel + fmt::Debug {
    fn box_clone(&self) -> Box<dyn DynLabel>;
}

impl PartialEq for dyn PageLabel {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn PageLabel {}

impl Hash for dyn PageLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl AsRef<dyn PageLabel> for dyn PageLabel {
    fn as_ref(&self) -> &dyn PageLabel {
        self
    }
}
