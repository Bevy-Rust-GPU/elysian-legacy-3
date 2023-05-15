//! Subtree domain
//!
//! Used to drill down into the leaf element of a shape tree

use crate::{DistanceF32, DistanceT, Domain, DomainF};

use type_fields::t_funk::{list::hlist::PushBackF, Curry2, Curry2B, Split};

pub enum Subtree {}

pub type SubtreeT<T> = <T as Domain<Subtree>>::Domain;
pub type SubtreeF = DomainF<Subtree>;

impl<T> Domain<Subtree> for T
where
    T: Clone + Domain<DistanceF32>,
    DistanceT<T>: Split<Curry2B<PushBackF, T>>,
{
    type Domain = Curry2B<PushBackF, T>;

    fn domain(self) -> Self::Domain {
        PushBackF.suffix2(self)
    }
}
