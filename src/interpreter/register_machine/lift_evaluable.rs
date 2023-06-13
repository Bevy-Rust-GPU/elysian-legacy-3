use t_funk::{
    closure::{CallF, Compose, ComposeLT},
    collection::set::{DropF, LiftContext, LiftContextT},
    macros::types,
    typeclass::arrow::{Fanout, FanoutT},
};

use crate::{EvaluateFunction, FunctionT, InputsT, LiftEvaluates, LiftEvaluatesT, MovesT};

#[types]
pub trait LiftEvaluable<A, D> {
    type LiftEvaluable;

    fn lift_evaluable(a: A) -> Self::LiftEvaluable;
}

// Use a C -> C function directly
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftNone;

impl<A, D> LiftEvaluable<A, D> for LiftNone
where
    A: EvaluateFunction<D>,
{
    type LiftEvaluable = FunctionT<A, D>;

    fn lift_evaluable(t: A) -> Self::LiftEvaluable {
        t.evaluate_function()
    }
}

// Lift an A -> B function to read and write from a context with move semantics
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftModify;

impl<A, D> LiftEvaluable<A, D> for LiftModify
where
    A: EvaluateFunction<D>,
    FunctionT<A, D>: LiftContext<InputsT<A, D>>,
    LiftContextT<FunctionT<A, D>, InputsT<A, D>>: Fanout<DropF<MovesT<A, D>>>,
{
    type LiftEvaluable = ComposeLT<
        FanoutT<LiftContextT<FunctionT<A, D>, InputsT<A, D>>, DropF<MovesT<A, D>>>,
        CallF,
    >;

    fn lift_evaluable(t: A) -> Self::LiftEvaluable {
        t.evaluate_function()
            .lift_context()
            .fanout(DropF::<MovesT<A, D>>::default())
            .compose_l(CallF)
    }
}

// Lift a function for each of the provided domains, and compose them into a fan-join structure
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftDomains;

impl<A, D> LiftEvaluable<A, D> for LiftDomains
where
    D: LiftEvaluates<A>,
{
    type LiftEvaluable = LiftEvaluatesT<D, A>;

    fn lift_evaluable(t: A) -> Self::LiftEvaluable {
        D::lift_evaluates(t)
    }
}
