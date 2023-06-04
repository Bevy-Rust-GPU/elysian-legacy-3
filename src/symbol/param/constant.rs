use t_funk::closure::Const;

use crate::{LiftAdt, LiftParam, Modify};

impl<T> LiftAdt for Const<T> {
    type LiftAdt = Modify<Self>;

    fn adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T> LiftParam for Const<T> {
    type LiftParam = Self;

    fn lift_param(self) -> Self::LiftParam {
        self
    }
}
