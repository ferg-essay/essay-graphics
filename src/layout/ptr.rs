// TODO: replace with downcast crate

use std::{any::TypeId, cell::UnsafeCell, mem::ManuallyDrop, ptr::NonNull};

struct Ptr {
    type_id: TypeId, 
    data: NonNull<u8>,
}

impl Ptr {
    fn new<T: 'static>(view: T) -> Self {
        let layout = std::alloc::Layout::new::<T>();
        let data = unsafe { std::alloc::alloc(layout) };
        let mut value = ManuallyDrop::new(view);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        let src = source.as_ptr();
        let count = std::mem::size_of::<T>();

        // TODO: drop
        
        unsafe {
            std::ptr::copy_nonoverlapping::<u8>(src, data, count);
        }

        Self {
            type_id: TypeId::of::<T>(),
            data: NonNull::new(data).unwrap(),
        }
    }

    #[inline]
    pub unsafe fn deref<T: 'static>(&self) -> &T {
        assert_eq!(self.type_id, TypeId::of::<T>());

        &*self.data.as_ptr().cast::<T>()
    }

    #[inline]
    pub unsafe fn deref_mut<T: 'static>(&self) -> &mut T {
        assert_eq!(self.type_id, TypeId::of::<T>());

        &mut *self.data.as_ptr().cast::<T>()
    }
}

// TODO: drop
