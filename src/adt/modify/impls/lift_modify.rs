use t_funk::{
    typeclass::category::{ComposeF, ComposeLT},
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{Combine, LiftAdtF, Modify, Sequence, Unit};

#[functions]
#[types]
pub trait LiftModify {
    type LiftModify;

    fn lift_modify(self) -> Self::LiftModify;
}

#[allow(non_snake_case)]
pub fn modify() -> DoUnit<ComposeLT<LiftModifyF, LiftAdtF>, ComposeF> {
    Default::default()
}

impl<A> LiftModify for Modify<A> {
    type LiftModify = Self;

    fn lift_modify(self) -> Self::LiftModify {
        self
    }
}

impl_adt! {
    impl<A, B, C> LiftModify for Unit<A> | Sequence<A, B> | Combine<A, B, C> {
        type LiftModify = Self;

        fn lift_modify(self) -> Self::LiftModify {
            self
        }
    }
}

