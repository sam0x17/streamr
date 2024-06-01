pub use super::*;

pub trait Node<T: Streamable = u8>: Default {
    fn state(&self) -> &Stream<Self, T>;
    fn state_mut(&mut self) -> &mut Stream<Self, T>;
}
