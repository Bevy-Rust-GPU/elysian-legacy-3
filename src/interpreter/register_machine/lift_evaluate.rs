use std::marker::PhantomData;

use t_funk::closure::{Closure, Compose, ComposeLT};

use crate::{
    Combine, Field, Input, LiftCombine, LiftCombineT, LiftDomains, LiftDomainsT, LiftModifier,
    LiftModifierT, Modify, Output, Sequence,
};

pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

pub type LiftEvaluateT<T, D> = <T as LiftEvaluate<D>>::LiftEvaluate;

impl<T, D> LiftEvaluate<D> for Input<T>
where
    D: LiftDomains<T>,
{
    type LiftEvaluate = LiftDomainsT<D, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        D::lift_domains(self.0)
    }
}

impl<T, D> LiftEvaluate<D> for Field<T>
where
    D: LiftDomains<T>,
{
    type LiftEvaluate = LiftDomainsT<D, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        D::lift_domains(self.0)
    }
}

impl<T, D> LiftEvaluate<D> for Output<T>
where
    D: LiftDomains<T>,
{
    type LiftEvaluate = LiftDomainsT<D, T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        D::lift_domains(self.0)
    }
}

impl<T, D> LiftEvaluate<D> for Modify<T>
where
    T: LiftModifier,
{
    type LiftEvaluate = LiftModifierT<T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_modifier()
    }
}

impl<A, B, D> LiftEvaluate<D> for Sequence<A, B>
where
    A: LiftEvaluate<D>,
    B: LiftEvaluate<D>,
    LiftEvaluateT<A, D>: Compose<LiftEvaluateT<B, D>>,
{
    type LiftEvaluate = ComposeLT<LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate().compose_l(self.1.lift_evaluate())
    }
}

impl<A, B, F, D> LiftEvaluate<D> for Combine<A, B, F>
where
    F: LiftCombine,
{
    type LiftEvaluate = LiftEvaluateCombine<A, B, LiftCombineT<F>, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftEvaluateCombine(self.0, self.1, self.2.lift_combine(), PhantomData)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftEvaluateCombine<A, B, F, D>(A, B, F, PhantomData<D>);

impl<A, B, F, D, C> Closure<C> for LiftEvaluateCombine<A, B, F, D>
where
    F: Closure<(A, B, C, PhantomData<D>), Output = C>,
{
    type Output = C;

    fn call(self, input: C) -> Self::Output {
        self.2.call((self.0, self.1, input, PhantomData))
    }
}
