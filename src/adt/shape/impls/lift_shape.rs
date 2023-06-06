use t_funk::{
    closure::ComposeLT,
    macros::{functions, impl_adt},
    r#do::DoUnit,
    typeclass::category::ComposeF,
};

use crate::{Combine, Field, Input, LiftAdtF, Output, Sequence, Unit};

#[functions]
pub trait LiftShape {
    type LiftShape;

    fn lift_shape(self) -> Self::LiftShape;
}

#[allow(non_snake_case)]
pub fn shape() -> DoUnit<ComposeLT<LiftShapeF, LiftAdtF>, ComposeF> {
    Default::default()
}

impl_adt! {
    impl<A, B> LiftShape for Input<A, B> | Field<A, B> | Output<A, B> {
        type LiftShape = Self;

        fn lift_shape(self) -> Self::LiftShape {
            self
        }
    }
}

impl_adt! {
    impl<A, B, C> LiftShape for Unit<A> | Sequence<A, B> | Combine<A, B, C> {
        type LiftShape = Self;

        fn lift_shape(self) -> Self::LiftShape {
            self
        }
    }
}
