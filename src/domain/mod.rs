mod distance;
mod evaluate;
mod gradient;
mod position;
mod subtree;

pub use distance::*;
pub use evaluate::*;
pub use gradient::*;
pub use position::*;
pub use subtree::*;

use type_fields::macros::functions;

#[functions]
pub trait Domain<T> {
    type Domain;

    fn domain(self) -> Self::Domain;
}

pub type DomainT<T, D> = <T as Domain<D>>::Domain;
