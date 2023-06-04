mod distance;
mod gradient;
mod position;
mod domains;

pub use distance::*;
pub use gradient::*;
pub use position::*;
pub use domains::*;

/// A type representing the lifting of a symbol into a function with a given set of outputs
pub trait Domain<T> {
    type Outputs;
}
