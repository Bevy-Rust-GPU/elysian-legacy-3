use t_funk::collection::set::Set;

use crate::LiftParam;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParamSet<T>(pub T);

impl<T, C> LiftParam<C> for ParamSet<T>
where
    C: Set<T>,
{
    type LiftParam = C;

    fn lift_param(self, input: C) -> Self::LiftParam {
        input.set(self.0)
    }
}
