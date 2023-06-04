use t_funk::{
    closure::{CallF, Curry2, Curry2B},
    closure::{Closure, Compose, ComposeLT, OutputT},
    typeclass::functor::{Fmap, FmapF, FmapT},
};

use crate::{LiftEvaluate, LiftEvaluateT, LiftParamF};

/// Given a list of domains, a shape, and a context,
/// evaluate the shape's domain functions and produce an updated context
pub trait Evaluate<D, C> {
    type Evaluate;

    fn evaluate(self, input: C) -> Self::Evaluate;
}

pub type EvaluateT<T, D, C> = <T as Evaluate<D, C>>::Evaluate;

impl<T, D, C> Evaluate<D, C> for T
where
    C: Clone,
    LiftParamF: Compose<Curry2B<CallF, C>>,
    T: Fmap<Curry2B<FmapF, ComposeLT<LiftParamF, Curry2B<CallF, C>>>>,
    FmapT<T, Curry2B<FmapF, ComposeLT<LiftParamF, Curry2B<CallF, C>>>>: LiftEvaluate<D>,
    LiftEvaluateT<FmapT<T, Curry2B<FmapF, ComposeLT<LiftParamF, Curry2B<CallF, C>>>>, D>:
        Closure<C>,
{
    type Evaluate = OutputT<
        LiftEvaluateT<FmapT<T, Curry2B<FmapF, ComposeLT<LiftParamF, Curry2B<CallF, C>>>>, D>,
        C,
    >;

    fn evaluate(self, input: C) -> Self::Evaluate {
        self.fmap(FmapF.suffix2(LiftParamF.compose_l(CallF.suffix2(input.clone()))))
            .lift_evaluate()
            .call(input)
    }
}
