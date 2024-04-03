use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicUsize};
    use super::*;

    #[test]
    fn test_read_copy_update_1() {
        let rcu = ReadCopyUpdate::new(1);
        let arc = rcu.read();
        assert_eq!(*arc, 1);
        rcu.update(2);
        let arc = rcu.read();
        assert_eq!(*arc, 2);
    }

    #[test]
    fn test_read_copy_update_2() {
        let atomic_counter = AtomicUsize::new(0);
        let should_break = Arc::new(AtomicBool::new(false));
        let should_break_copy = should_break.clone();
        let rcu = Arc::new(ReadCopyUpdate::new(1));
        let rcu_copy = rcu.clone();

        // busy loop
        let _loop_spawn = std::thread::spawn(move || {
            loop {
                if should_break.load(Ordering::Relaxed) {
                    break;
                }
                let _value = rcu_copy.read();
                atomic_counter.fetch_add(1, Ordering::Relaxed);
            }
        });

        // update
        for i in 0..100 {
            rcu.update(i);
            let value = rcu.read();
            assert_eq!(*value, i);
        }

        should_break_copy.store(true, Ordering::Relaxed);
    }
}