use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{symbol::Intersection as IntersectionS, Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

pub fn intersection() -> OpChain<LiftAdtF, IntersectionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Intersection<R> for Then<A, B> | Combine<A, B, C> {
        type Intersection = Combine<Self, R, IntersectionS>;

        fn intersection(self, rhs: R) -> Self::Intersection {
            Combine(self, rhs, IntersectionS)
        }
    }
}
