use core::{fmt::Debug, hash::Hash};

pub trait Streamable: Send + Copy + Clone + Debug + Hash + PartialEq + Eq {}

impl<T> Streamable for T where T: Send + Copy + Clone + Debug + Hash + PartialEq + Eq {}
