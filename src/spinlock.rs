use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    // Method to check the lock state for testing purposes
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }
}

#[test]
fn test_lock_unlock() {
    let lock = Spinlock::new();

    lock.lock();
    assert!(lock.is_locked());

    lock.unlock();
    assert!(!lock.is_locked());
}

#[test]
fn test_double_lock() {
    let lock = Spinlock::new();

    lock.lock();
    assert!(lock.is_locked());

    lock.unlock();
    assert!(!lock.is_locked());

    lock.lock();
    assert!(lock.is_locked());

    lock.unlock();
    assert!(!lock.is_locked());
}

#[test]
fn test_lock_unlock_multiple() {
    let lock = Spinlock::new();

    for _ in 0..10 {
        lock.lock();
        assert!(lock.is_locked());

        lock.unlock();
        assert!(!lock.is_locked());
    }
}
