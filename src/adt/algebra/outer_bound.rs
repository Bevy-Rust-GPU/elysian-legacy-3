use t_funk::{
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{symbol::OuterBound as OuterBoundS, Combine, Field, LiftAdtF, Sequence};

#[functions]
#[types]
pub trait OuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

pub fn outer_bound() -> DoUnit<LiftAdtF, OuterBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> OuterBound<R> for Field<A, B> | Sequence<A, B> | Combine<A, B, C> {
        type OuterBound = Combine<Self, R, OuterBoundS>;

        fn outer_bound(self, rhs: R) -> Self::OuterBound {
            Combine(self, rhs, OuterBoundS)
        }
    }
}
