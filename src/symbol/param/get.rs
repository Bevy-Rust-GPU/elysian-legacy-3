use crate::{Get, LiftParam};

impl<T, C> LiftParam<C> for Get<T>
where
    C: t_funk::collection::set::Get<T>,
{
    type LiftParam = T;

    fn lift_param(self, input: C) -> Self::LiftParam {
        input.get()
    }
}
