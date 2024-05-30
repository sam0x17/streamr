use core::{fmt::Debug, hash::Hash, marker::PhantomData};

pub trait Streamable: Copy + Clone + Debug + Hash + PartialEq + Eq {}

impl<T> Streamable for T where T: Copy + Clone + Debug + Hash + PartialEq + Eq {}

pub trait Node<T: Streamable = u8>: Default {
    fn state(&self) -> &Stream<Self, T>;
    fn state_mut(&mut self) -> &mut Stream<Self, T>;
}

pub struct 

pub struct Stream<N: Node<T>, T: Streamable = u8> {
    node: N,
    limit: Option<u64>,
    remaining: Option<u64>,
    _phantom: PhantomData<T>,
}
