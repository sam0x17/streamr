use super::*;

use core::marker::PhantomData;

pub struct Stream<N: Node<T>, T: Streamable = u8> {
    node: N,
    limit: Option<u64>,
    remaining: Option<u64>,
    _phantom: PhantomData<T>,
}

impl<N: Node<T>, T: Streamable> Stream<N, T> {
    pub fn new() -> Self {
        Self {
            node: N::default(),
            limit: None,
            remaining: None,
            _phantom: PhantomData,
        }
    }
}
