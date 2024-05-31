use crate::{ring_buffer::RingBuffer, spinlock::Spinlock};
use core::cell::UnsafeCell;

pub struct Channel<T, const N: usize> {
    buffer: UnsafeCell<RingBuffer<T, N>>,
    lock: Spinlock,
}

impl<T: Clone, const N: usize> Channel<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(RingBuffer::new()),
            lock: Spinlock::new(),
        }
    }

    pub fn send(&self, item: T) {
        loop {
            self.lock.lock();
            let result = unsafe { &mut *self.buffer.get() }.push(&item);
            self.lock.unlock();

            if result.is_ok() {
                break;
            }
            core::hint::spin_loop();
        }
    }

    pub fn receive(&self) -> T {
        loop {
            self.lock.lock();
            let item = unsafe { &mut *self.buffer.get() }.pop();
            self.lock.unlock();

            if let Some(value) = item {
                return value;
            }
            core::hint::spin_loop();
        }
    }
}
