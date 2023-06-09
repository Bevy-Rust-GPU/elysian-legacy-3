use t_funk::macros::impl_adt;

use crate::{Field, Input, LiftAdt, Run, Output, ShapeEnd, AdtEnd};

impl_adt! {
    impl<A, B> LiftAdt for Input<A, B> | Field<A, B> | Output<A, B> {
        type LiftAdt = Run<Self>;

        fn lift_adt(self) -> Self::LiftAdt {
            Run(self)
        }
    }
}

impl LiftAdt for ShapeEnd {
    type LiftAdt = AdtEnd;

    fn lift_adt(self) -> Self::LiftAdt {
        AdtEnd
    }
}
