use core::{fmt::Debug, hash::Hash, marker::PhantomData};

pub trait Streamable: Copy + Clone + Debug + Hash + PartialEq + Eq {}

impl<T> Streamable for T where T: Copy + Clone + Debug + Hash + PartialEq + Eq {}

pub trait Node<T: Streamable = u8>: Default {
    fn state(&self) -> &Stream<Self, T>;
    fn state_mut(&mut self) -> &mut Stream<Self, T>;
}

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
