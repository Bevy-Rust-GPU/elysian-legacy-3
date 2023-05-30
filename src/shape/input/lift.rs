//! Lift a single input (ex. position) into the expected input type of a composite domain

use crate::{impl_identity, impl_null, DistanceF32, Domain, DomainT, GradientF32, Split, Subtree, impl_domains};
use type_fields::t_funk::{
    function::Const, function::Id, hlist::Nil, Curry2, Curry2A, Fanout, Fanouted,
};

// TODO: This should probably defer to a LiftTrait impl on the domain types themselves,
//       instead of being implemented for concrete types,
//       since this prevents downstream crates from creating their of lifting impls.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lift;

impl<A, B> Domain<Split<A, B>> for Lift
where
    Lift: Domain<B>,
{
    type Input = ();
    type Domain = Fanouted<Id, DomainT<Lift, B>>;

    fn domain(self) -> Self::Domain {
        Id.fanout(Domain::<B>::domain(self))
    }
}

impl Domain<DistanceF32> for Lift {
    type Input = ();
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

impl Domain<GradientF32> for Lift {
    type Input = ();
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

impl Domain<Subtree> for Lift {
    type Input = ();
    type Domain = Curry2A<Const, Nil>;

    fn domain(self) -> Self::Domain {
        Const.prefix2(Nil)
    }
}

impl_identity!(Lift);
impl_domains!(Lift);
impl_null!(Lift);
