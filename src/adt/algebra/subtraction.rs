use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{symbol::Subtraction as SubtractionS, Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Subtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

pub fn subtraction() -> OpChain<LiftAdtF, SubtractionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Subtraction<R> for Then<A, B> | Combine<A, B, C> {
        type Subtraction = Combine<Self, R, SubtractionS>;

        fn subtraction(self, rhs: R) -> Self::Subtraction {
            Combine(self, rhs, SubtractionS)
        }
    }
}
