use t_funk::{
    macros::{functions, impl_adt},
    op_chain::{op_chain_lift, OpChain},
    typeclass::category::ComposeF,
};

use crate::{Run, Combine, Then, AdtEnd};

#[functions]
pub trait LiftAdt {
    type LiftAdt;

    fn lift_adt(self) -> Self::LiftAdt;
}

#[allow(non_snake_case)]
pub fn adt() -> OpChain<LiftAdtF, ComposeF> {
    op_chain_lift(LiftAdtF, ComposeF)
}

pub type LiftAdtT<T> = <T as LiftAdt>::LiftAdt;

impl_adt! {
    impl<A, B, C> LiftAdt for AdtEnd | Run<A> | Then<A, B> | Combine<A, B, C> {
        type LiftAdt = Self;

        fn lift_adt(self) -> Self::LiftAdt {
            self
        }
    }
}
