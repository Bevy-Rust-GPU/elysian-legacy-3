use t_funk::{
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{symbol::InnerBound as InnerBoundS, Combine, Field, LiftAdtF, Sequence};

#[functions]
#[types]
pub trait InnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

pub fn inner_bound() -> DoUnit<LiftAdtF, InnerBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> InnerBound<R> for Field<A, B> | Sequence<A, B> | Combine<A, B, C> {
        type InnerBound = Combine<Self, R, InnerBoundS>;

        fn inner_bound(self, rhs: R) -> Self::InnerBound {
            Combine(self, rhs, InnerBoundS)
        }
    }
}
