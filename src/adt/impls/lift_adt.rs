use t_funk::{
    macros::{functions, impl_adt},
    r#do::{do_lift, DoUnit},
    typeclass::category::ComposeF,
};

use crate::{Combine, Sequence, Unit, Nil};

#[functions]
pub trait LiftAdt {
    type LiftAdt;

    fn lift_adt(self) -> Self::LiftAdt;
}

#[allow(non_snake_case)]
pub fn adt() -> DoUnit<LiftAdtF, ComposeF> {
    do_lift(LiftAdtF, ComposeF)
}

pub type LiftAdtT<T> = <T as LiftAdt>::LiftAdt;

impl_adt! {
    impl<A, B, C> LiftAdt for Nil | Unit<A> | Sequence<A, B> | Combine<A, B, C> {
        type LiftAdt = Self;

        fn lift_adt(self) -> Self::LiftAdt {
            self
        }
    }
}
