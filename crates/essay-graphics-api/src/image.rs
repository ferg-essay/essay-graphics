use std::sync::Arc;

#[derive(Clone)]
pub struct ImageId(Arc<ImageIndex>);

impl ImageId {
    pub fn new(id: usize) -> ImageId {
        ImageId(Arc::new(ImageIndex(id)))
    }

    #[inline]
    pub fn is_live(&self) -> bool {
        Arc::strong_count(&self.0) > 1
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.0.index()
    }
}

impl PartialEq for ImageId {
    fn eq(&self, other: &Self) -> bool {
        self.0.index() == other.0.index()
    }
}

pub struct ImageIndex(usize);

impl ImageIndex {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}
