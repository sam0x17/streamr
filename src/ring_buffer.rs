pub struct RingBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    read_idx: usize,
    write_idx: usize,
    full: bool,
}

impl<T: Clone, const N: usize> RingBuffer<T, N> {
    const ARRAY_REPEAT_VALUE: Option<T> = None;

    pub const fn new() -> Self {
        Self {
            buffer: [Self::ARRAY_REPEAT_VALUE; N],
            read_idx: 0,
            write_idx: 0,
            full: false,
        }
    }

    pub fn push(&mut self, item: &T) -> Result<(), ()> {
        if self.full {
            return Err(());
        }

        self.buffer[self.write_idx] = Some(item.clone());
        self.write_idx = (self.write_idx + 1) % N;
        if self.write_idx == self.read_idx {
            self.full = true;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.read_idx].take();
        self.read_idx = (self.read_idx + 1) % N;
        self.full = false;
        item
    }

    pub fn is_empty(&self) -> bool {
        !self.full && (self.read_idx == self.write_idx)
    }
}

#[test]
fn test_push_pop_single_element() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    assert!(buffer.is_empty());

    // Push a single element
    assert!(buffer.push(&1).is_ok());
    assert!(!buffer.is_empty());

    // Pop the single element
    assert_eq!(buffer.pop(), Some(1));
    assert!(buffer.is_empty());
}

#[test]
fn test_push_until_full() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Fill the buffer
    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());

    // Buffer should be full now
    assert!(buffer.push(&4).is_err());
}

#[test]
fn test_pop_until_empty() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Fill the buffer
    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());

    // Pop all elements
    assert_eq!(buffer.pop(), Some(1));
    assert_eq!(buffer.pop(), Some(2));
    assert_eq!(buffer.pop(), Some(3));

    // Buffer should be empty now
    assert!(buffer.pop().is_none());
}

#[test]
fn test_wrap_around() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Fill the buffer
    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());

    // Pop two elements
    assert_eq!(buffer.pop(), Some(1));
    assert_eq!(buffer.pop(), Some(2));

    // Add more elements to wrap around
    assert!(buffer.push(&4).is_ok());
    assert!(buffer.push(&5).is_ok());

    // Pop remaining elements
    assert_eq!(buffer.pop(), Some(3));
    assert_eq!(buffer.pop(), Some(4));
    assert_eq!(buffer.pop(), Some(5));
    assert!(buffer.is_empty());
}

#[test]
fn test_is_empty() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Initially empty
    assert!(buffer.is_empty());

    // Add elements
    assert!(buffer.push(&1).is_ok());
    assert!(!buffer.is_empty());

    // Remove elements
    assert_eq!(buffer.pop(), Some(1));
    assert!(buffer.is_empty());
}

#[test]
fn test_is_full() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Fill the buffer
    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());

    // Buffer should be full now
    assert!(buffer.push(&4).is_err());
}

#[test]
fn test_push_pop_different_types() {
    let mut buffer: RingBuffer<&str, 3> = RingBuffer::new();

    assert!(buffer.push(&"hello").is_ok());
    assert_eq!(buffer.pop(), Some("hello"));

    let mut buffer: RingBuffer<(i32, i32), 3> = RingBuffer::new();
    assert!(buffer.push(&(1, 2)).is_ok());
    assert_eq!(buffer.pop(), Some((1, 2)));
}

#[test]
fn test_push_pop_alternation() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    assert!(buffer.push(&1).is_ok());
    assert_eq!(buffer.pop(), Some(1));
    assert!(buffer.push(&2).is_ok());
    assert_eq!(buffer.pop(), Some(2));
    assert!(buffer.push(&3).is_ok());
    assert_eq!(buffer.pop(), Some(3));
}

#[test]
fn test_overwriting_elements() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());
    assert!(buffer.push(&4).is_err()); // Buffer is full, should fail

    assert_eq!(buffer.pop(), Some(1));
    assert!(buffer.push(&4).is_ok()); // Now there's space, should succeed

    assert_eq!(buffer.pop(), Some(2));
    assert_eq!(buffer.pop(), Some(3));
    assert_eq!(buffer.pop(), Some(4));
}

#[test]
fn test_empty_after_full() {
    let mut buffer: RingBuffer<u32, 3> = RingBuffer::new();

    // Fill the buffer
    assert!(buffer.push(&1).is_ok());
    assert!(buffer.push(&2).is_ok());
    assert!(buffer.push(&3).is_ok());

    // Pop all elements
    assert_eq!(buffer.pop(), Some(1));
    assert_eq!(buffer.pop(), Some(2));
    assert_eq!(buffer.pop(), Some(3));

    // Buffer should be empty now
    assert!(buffer.is_empty());
    assert!(buffer.push(&4).is_ok());
    assert_eq!(buffer.pop(), Some(4));
}
