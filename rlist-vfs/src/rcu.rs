use std::cell::UnsafeCell;
use std::sync::Arc;

pub struct ReadCopyUpdate<T> (UnsafeCell<Arc<T>>);

impl<T> ReadCopyUpdate<T> {
    pub fn new(value: T) -> Self {
        ReadCopyUpdate(UnsafeCell::new(Arc::new(value)))
    }

    pub fn read(&self) -> Arc<T> {
        unsafe { (*self.0.get()).clone() }
    }

    pub fn update(&self, value: T) {
        let new = Arc::new(value);
        unsafe { *self.0.get() = new; }
    }
}

unsafe impl<T> Sync for ReadCopyUpdate<T> {}