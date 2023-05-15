//! Evaluation domain
//!
//! Subdomain of distance, composes with Subtree to create an evaluation context

use crate::{DistanceF, DistanceF32, DistanceT, Domain, DomainF, Subtree, SubtreeF, SubtreeT};

use type_fields::t_funk::{Closure, Split, SplitT};

pub enum Evaluate {}

pub type EvaluateT<T> = <T as Domain<Evaluate>>::Domain;
pub type EvaluateF = DomainF<Evaluate>;

impl<T> Domain<Evaluate> for T
where
    T: Clone + Domain<DistanceF32> + Domain<Subtree>,
    DistanceT<T>: Split<SubtreeT<T>>,
{
    type Domain = SplitT<DistanceT<T>, SubtreeT<T>>;

    fn domain(self) -> Self::Domain {
        DistanceF::default()
            .call(self.clone())
            .split(SubtreeF::default().call(self))
    }
}
