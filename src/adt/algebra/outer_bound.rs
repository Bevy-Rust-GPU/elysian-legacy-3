use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{symbol::OuterBound as OuterBoundS, Combine, LiftAdtF, Then, Run};

#[functions]
#[types]
pub trait OuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

pub fn outer_bound() -> OpChain<LiftAdtF, OuterBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> OuterBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type OuterBound = Combine<Self, R, OuterBoundS>;

        fn outer_bound(self, rhs: R) -> Self::OuterBound {
            Combine(self, rhs, OuterBoundS)
        }
    }
}
