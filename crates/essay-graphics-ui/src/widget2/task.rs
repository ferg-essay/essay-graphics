use std::marker::PhantomData;

#[must_use]
pub struct Task<T> {
    _message: PhantomData<fn(T)>,
}

impl<T> Task<T> {
    pub fn none() -> Self {
        Self {
            _message: Default::default(),
        }
    }
}