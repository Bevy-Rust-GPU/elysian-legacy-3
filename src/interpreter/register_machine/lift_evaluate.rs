use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, Compose, ComposeLT, OutputT},
    collection::map::{Get, GetT},
    macros::{functions, types},
};

use crate::{
    AdtEnd, Combine, CombineContext, ContextOut, Evaluable, LiftEvaluable, LiftEvaluableT, LiftT,
    NotAdtEnd, Run, Then,
};

#[functions]
#[types]
pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

impl<A, D> LiftEvaluate<D> for Run<A>
where
    A: Evaluable,
    LiftT<A>: LiftEvaluable<A, D>,
{
    type LiftEvaluate = LiftEvaluableT<LiftT<A>, A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftT::<A>::lift_evaluable(self.0)
    }
}

impl<A, B, D> LiftEvaluate<D> for Then<A, B>
where
    A: LiftEvaluate<D>,
    B: LiftEvaluate<D>,
    LiftEvaluateT<A, D>: Compose<LiftEvaluateT<B, D>>,
    B: NotAdtEnd,
{
    type LiftEvaluate = ComposeLT<LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate().compose_l(self.1.lift_evaluate())
    }
}

impl<A, D> LiftEvaluate<D> for Then<A, AdtEnd>
where
    A: LiftEvaluate<D>,
{
    type LiftEvaluate = LiftEvaluateT<A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate()
    }
}

impl<A, B, F, D> LiftEvaluate<D> for Combine<A, B, F> {
    type LiftEvaluate = LiftEvaluateCombine<A, B, F, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftEvaluateCombine(self.0, self.1, self.2, PhantomData)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftEvaluateCombine<A, B, F, D>(pub A, pub B, pub F, pub PhantomData<D>);

impl<A, B, F, D, CI> Closure<CI> for LiftEvaluateCombine<A, B, F, D>
where
    A: Clone + LiftEvaluate<D>,
    B: Clone + LiftEvaluate<D>,
    F: Closure<CombineContext<A, B, CI, (), (), (), LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>>,
    OutputT<F, CombineContext<A, B, CI, (), (), (), LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>>:
        Get<ContextOut>,
{
    type Output = GetT<
        OutputT<F, CombineContext<A, B, CI, (), (), (), LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>>,
        ContextOut,
    >;

    fn call(self, input: CI) -> Self::Output {
        self.2
            .call(CombineContext {
                shape_a: self.0.clone(),
                shape_b: self.1.clone(),
                context_in: input,
                context_a: (),
                context_b: (),
                context_out: (),
                inherited_a: LiftEvaluate::<D>::lift_evaluate(self.0),
                inherited_b: LiftEvaluate::<D>::lift_evaluate(self.1),
            })
            .get()
    }
}
