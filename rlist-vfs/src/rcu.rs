use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct ReadCopyUpdate<T> (AtomicPtr<Arc<T>>);

impl<T> ReadCopyUpdate<T> {
    pub fn new(value: T) -> Self {
        ReadCopyUpdate(
            AtomicPtr::new(
                Box::into_raw(
                    Box::new(
                        Arc::new(value)
                    )
                )
            )
        )
    }

    pub fn read(&self) -> Arc<T> {
        unsafe {
            Arc::clone(&*self.0.load(Ordering::Relaxed))
        }
    }

    pub fn update(&self, value: T) {
        let new = Arc::new(value);
        let old = self.0.swap(
            Box::into_raw(Box::new(new)),
            Ordering::Relaxed
        );
        unsafe {
            drop(Box::from_raw(old));
        }
    }
}

unsafe impl<T> Sync for ReadCopyUpdate<T> {}