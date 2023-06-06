use crate::{Field, Input, LiftAdt, Output, Shape};
use t_funk::macros::impl_adt;

impl_adt! {
    impl<A, B> LiftAdt for Input<A, B> | Field<A, B> | Output<A, B> {
        type LiftAdt = Shape<Self>;

        fn lift_adt(self) -> Self::LiftAdt {
            Shape(self)
        }
    }
}
