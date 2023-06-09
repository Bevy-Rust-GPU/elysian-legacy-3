use t_funk::{
    closure::{Closure, OutputT},
    closure::{Curry2, Curry2B},
    collection::set::SetF,
    typeclass::functor::Fmap,
};

use crate::{LiftAdt, LiftEvaluate, LiftParam, LiftParamT, Run};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContextSet<T>(pub T);

impl<T, F> Fmap<F> for ContextSet<T>
where
    F: Closure<T>,
{
    type Fmap = ContextSet<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        ContextSet(f.call(self.0))
    }
}

impl<T> LiftAdt for ContextSet<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, D> LiftEvaluate<D> for ContextSet<T> {
    type LiftEvaluate = Curry2B<SetF, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        SetF.suffix2(self.0)
    }
}

impl<T, C> LiftParam<C> for ContextSet<T>
where
    T: LiftParam<C>,
{
    type LiftParam = ContextSet<LiftParamT<T, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        ContextSet(self.0.lift_param(input))
    }
}
