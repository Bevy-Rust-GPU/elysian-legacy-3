use t_funk::closure::Const;

use crate::{LiftAdt, LiftParam, Modify};

impl<T> LiftAdt for Const<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, C> LiftParam<C> for Const<T> {
    type LiftParam = T;

    fn lift_param(self, _: C) -> Self::LiftParam {
        self.0
    }
}
