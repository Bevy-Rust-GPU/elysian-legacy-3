use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{symbol::InnerBound as InnerBoundS, Combine, LiftAdtF, Then, Run};

#[functions]
#[types]
pub trait InnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

pub fn inner_bound() -> OpChain<LiftAdtF, InnerBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> InnerBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type InnerBound = Combine<Self, R, InnerBoundS>;

        fn inner_bound(self, rhs: R) -> Self::InnerBound {
            Combine(self, rhs, InnerBoundS)
        }
    }
}
