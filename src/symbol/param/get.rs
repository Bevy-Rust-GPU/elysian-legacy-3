use t_funk::collection::set::GetF;

use crate::{Get, LiftParam};

impl<T> LiftParam for Get<T> {
    type LiftParam = GetF<T>;

    fn lift_param(self) -> Self::LiftParam {
        GetF::<T>::default()
    }
}
