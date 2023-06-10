mod impls;

pub use impls::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modify<T>(pub T);
