use crate::{Field, Input, LiftAdt, Output, Unit};
use t_funk::macros::impl_adt;

impl_adt! {
    impl<A> LiftAdt for Input<A> | Field<A> | Output<A> {
        type LiftAdt = Unit<Self>;

        fn lift_adt(self) -> Self::LiftAdt {
            Unit(self)
        }
    }
}
