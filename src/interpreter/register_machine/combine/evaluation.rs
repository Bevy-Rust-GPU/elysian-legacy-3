use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::{
        map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
    },
};

use crate::{LiftEvaluate, LiftEvaluateT, Pair, ShapeA, ShapeB};

use super::{ContextIn, InheritedA, InheritedB};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Left;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Right;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inherited;

// Evaluate side S with domains I (or Inherited), and store output in O
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluateSide<S, I, O>(pub PhantomData<(S, I, O)>);

impl<S, I, O, D> LiftEvaluate<D> for EvaluateSide<S, I, O> {
    type LiftEvaluate = (Self,);

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        (self,)
    }
}

impl<O, C> Closure<C> for EvaluateSide<Left, Inherited, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<InheritedA>
        + InsertM<O, OutputT<GetMT<C, InheritedA>, GetMT<C, ContextIn>>>,
    GetMT<C, InheritedA>: Closure<GetMT<C, ContextIn>>,
{
    type Output = InsertMT<C, O, OutputT<GetMT<C, InheritedA>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let inherited_a = GetM::<InheritedA>::get(ctx.clone());
        ctx.insert(inherited_a.call(context_in))
    }
}

impl<D, O, C> Closure<C> for EvaluateSide<Left, D, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<ShapeA>
        + InsertM<O, OutputT<LiftEvaluateT<GetMT<C, ShapeA>, D>, GetMT<C, ContextIn>>>,
    GetMT<C, ShapeA>: LiftEvaluate<D>,
    LiftEvaluateT<GetMT<C, ShapeA>, D>: Closure<GetMT<C, ContextIn>>,
    D: Pair,
{
    type Output = InsertMT<C, O, OutputT<LiftEvaluateT<GetMT<C, ShapeA>, D>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let shape_a = GetM::<ShapeA>::get(ctx.clone());
        ctx.insert(shape_a.lift_evaluate().call(context_in))
    }
}

impl<O, C> Closure<C> for EvaluateSide<Right, Inherited, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<InheritedB>
        + InsertM<O, OutputT<GetMT<C, InheritedB>, GetMT<C, ContextIn>>>,
    GetMT<C, InheritedB>: Closure<GetMT<C, ContextIn>>,
{
    type Output = InsertMT<C, O, OutputT<GetMT<C, InheritedB>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let inherited_b = GetM::<InheritedB>::get(ctx.clone());
        ctx.insert(inherited_b.call(context_in))
    }
}

impl<D, O, C> Closure<C> for EvaluateSide<Right, D, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<ShapeB>
        + InsertM<O, OutputT<LiftEvaluateT<GetMT<C, ShapeB>, D>, GetMT<C, ContextIn>>>,
    GetMT<C, ShapeB>: LiftEvaluate<D>,
    LiftEvaluateT<GetMT<C, ShapeB>, D>: Closure<GetMT<C, ContextIn>>,
    D: Pair,
{
    type Output = InsertMT<C, O, OutputT<LiftEvaluateT<GetMT<C, ShapeB>, D>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let shape_b = GetM::<ShapeB>::get(ctx.clone());
        ctx.insert(shape_b.lift_evaluate().call(context_in))
    }
}
