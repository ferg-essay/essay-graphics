// From egui id.rs
use core::{fmt, hash};
use std::num::NonZeroU64;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id(NonZeroU64);

impl Id {
    pub const NULL: Self = Self(NonZeroU64::MAX);

    #[inline]
    const fn from_hash(hash: u64) -> Self {
        if let Some(nonzero) = NonZeroU64::new(hash) {
            Self(nonzero)
        } else {
            Self(NonZeroU64::MIN)
        }
    }

    pub fn new(source: impl hash::Hash) -> Self {
        Self::from_hash(ahash::RandomState::with_seeds(1, 2, 3, 4).hash_one(source))
    }

    pub fn with(self, child: impl std::hash::Hash) -> Self {
        use std::hash::{BuildHasher, Hasher};

        let mut hasher = ahash::RandomState::with_seeds(1, 2, 3, 4)
            .build_hasher();
        hasher.write_u64(self.0.get());
        child.hash(&mut hasher);
        Self::from_hash(hasher.finish())
    }

    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.0.get()
    }

    pub fn short_debug_format(&self) -> String {
        format!("{:04X}", self.value() as u16)
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.value() as u16)
    }
}

impl nohash_hasher::IsEnabled for Id {}

impl From<&'static str> for Id {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

pub type IdSet = nohash_hasher::IntSet<Id>;
pub type IdMap<V> = nohash_hasher::IntMap<Id, V>;

#[cfg(test)]
mod test {
    use crate::util::Id;

    #[test]
    fn id_size() {
        assert_eq!(std::mem::size_of::<Id>(), 8);
        assert_eq!(std::mem::size_of::<Option<Id>>(), 8);
    }
}