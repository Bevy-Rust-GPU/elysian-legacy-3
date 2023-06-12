use crate::{Shape, LiftAdt, Run};

impl<A> LiftAdt for Shape<A> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}
