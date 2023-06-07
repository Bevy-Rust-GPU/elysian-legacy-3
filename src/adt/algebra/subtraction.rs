use t_funk::{
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{symbol::Subtraction as SubtractionS, Combine, Field, LiftAdtF, Sequence};

#[functions]
#[types]
pub trait Subtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

pub fn subtraction() -> DoUnit<LiftAdtF, SubtractionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Subtraction<R> for Field<A> | Sequence<A, B> | Combine<A, B, C> {
        type Subtraction = Combine<Self, R, SubtractionS>;

        fn subtraction(self, rhs: R) -> Self::Subtraction {
            Combine(self, rhs, SubtractionS)
        }
    }
}
