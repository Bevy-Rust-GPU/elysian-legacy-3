use t_funk::closure::Const;

use crate::{LiftAdt, LiftParam, Run};

impl<T> LiftAdt for Const<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, C> LiftParam<C> for Const<T> {
    type LiftParam = T;

    fn lift_param(self, _: C) -> Self::LiftParam {
        self.0
    }
}
