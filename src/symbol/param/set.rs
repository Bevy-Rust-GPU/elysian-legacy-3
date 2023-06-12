use t_funk::collection::set::{Insert, InsertT};

use crate::LiftParam;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParamSet<T>(pub T);

impl<T, C> LiftParam<C> for ParamSet<T>
where
    C: Insert<T>,
{
    type LiftParam = InsertT<C, T>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        input.insert(self.0)
    }
}
