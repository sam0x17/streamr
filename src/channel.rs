use crate::{ring_buffer::RingBuffer, spinlock::Spinlock};
use core::cell::UnsafeCell;
use core::hint::spin_loop;

const MAX_BACKOFF: usize = 1024; // Maximum backoff value

pub struct Channel<T, const N: usize> {
    buffer: UnsafeCell<RingBuffer<T, N>>,
    lock: Spinlock,
}

unsafe impl<T: Clone + Send, const N: usize> Sync for Channel<T, N> {}

impl<T: Clone, const N: usize> Channel<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(RingBuffer::new()),
            lock: Spinlock::new(),
        }
    }

    pub fn send(&self, item: T) {
        let mut backoff = 1;
        loop {
            self.lock.lock();
            let result = unsafe { &mut *self.buffer.get() }.push(&item);
            self.lock.unlock();

            if result.is_ok() {
                break;
            }
            for _ in 0..backoff {
                spin_loop();
            }
            backoff = (backoff * 2).min(MAX_BACKOFF); // Use the MAX_BACKOFF constant
        }
    }

    pub fn receive(&self) -> T {
        let mut backoff = 1;
        loop {
            self.lock.lock();
            let item = unsafe { &mut *self.buffer.get() }.pop();
            self.lock.unlock();

            if let Some(value) = item {
                return value;
            }
            for _ in 0..backoff {
                spin_loop();
            }
            backoff = (backoff * 2).min(MAX_BACKOFF); // Use the MAX_BACKOFF constant
        }
    }
}

// Inline tests for Channel

#[test]
fn test_send_receive_single_element() {
    let channel: Channel<u32, 3> = Channel::new();

    channel.send(1);
    let result = channel.receive();
    assert_eq!(result, 1);
}
