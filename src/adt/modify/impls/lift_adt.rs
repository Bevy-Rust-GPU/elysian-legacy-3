use crate::{LiftAdt, Modify, Run};

impl<T> LiftAdt for Modify<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

