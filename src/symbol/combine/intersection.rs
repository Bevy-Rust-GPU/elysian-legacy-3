use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Gt,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, Left, LiftEvaluate, Pair, Right,
};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

pub fn intersection() -> OpChain<LiftAdtF, IntersectionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Intersection<R> for Then<A, B> | Combine<A, B, C> {
        type Intersection = Combine<Self, R, IntersectionS>;

        fn intersection(self, rhs: R) -> Self::Intersection {
            Combine(self, rhs, IntersectionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntersectionS;

impl LiftEvaluate<Dist<f32>> for IntersectionS {
    type LiftEvaluate = ComposeT<
        BooleanConditional<
            Gt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
        ComposeT<EvaluateSide<Right, Inherited, ContextB>, EvaluateSide<Left, Inherited, ContextA>>,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluateSide::<Left, Inherited, ContextA>::default()
            .compose_l(EvaluateSide::<Right, Inherited, ContextB>::default())
            .compose_l(BooleanConditional(
                Gt,
                CopyContext::default(),
                CopyContext::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for IntersectionS
where
    D: Pair,
{
    type LiftEvaluate = ComposeT<
        BooleanConditional<
            Gt,
            EvaluateSide<Left, Inherited, ContextOut>,
            EvaluateSide<Right, Inherited, ContextOut>,
            Distance<f32>,
        >,
        ComposeT<EvaluateSide<Right, Dist<f32>, ContextB>, EvaluateSide<Left, Dist<f32>, ContextA>>,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluateSide::<Left, Dist<f32>, ContextA>::default()
            .compose_l(EvaluateSide::<Right, Dist<f32>, ContextB>::default())
            .compose_l(BooleanConditional(
                Gt,
                EvaluateSide::<Left, Inherited, ContextOut>::default(),
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}
