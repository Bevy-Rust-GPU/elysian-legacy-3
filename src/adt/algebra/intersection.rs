use t_funk::{
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{symbol::Intersection as IntersectionS, Combine, Field, LiftAdtF, Sequence};

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

pub fn intersection() -> DoUnit<LiftAdtF, IntersectionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Intersection<R> for Field<A> | Sequence<A, B> | Combine<A, B, C> {
        type Intersection = Combine<Self, R, IntersectionS>;

        fn intersection(self, rhs: R) -> Self::Intersection {
            Combine(self, rhs, IntersectionS)
        }
    }
}
