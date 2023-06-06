use crate::{LiftParam, Set};

impl<T, C> LiftParam<C> for Set<T>
where
    C: t_funk::collection::set::Set<T>,
{
    type LiftParam = C;

    fn lift_param(self, input: C) -> Self::LiftParam {
        input.set(self.0)
    }
}
