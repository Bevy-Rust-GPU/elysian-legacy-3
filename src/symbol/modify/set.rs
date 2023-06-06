use t_funk::{
    closure::{Closure, OutputT},
    closure::{Curry2, Curry2B},
    collection::set::SetF,
    typeclass::functor::Fmap,
};

use crate::{LiftEvaluate, LiftModify, Modify};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set<T>(pub T);

impl<T, F> Fmap<F> for Set<T>
where
    F: Closure<T>,
{
    type Fmap = Set<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Set(f.call(self.0))
    }
}

impl<T> LiftModify for Set<T> {
    type LiftModify = Modify<Self>;

    fn lift_modify(self) -> Self::LiftModify {
        Modify(self)
    }
}

impl<T, D> LiftEvaluate<D> for Set<T> {
    type LiftEvaluate = Curry2B<SetF, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        SetF.suffix2(self.0)
    }
}
