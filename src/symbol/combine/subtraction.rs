use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT, Composed},
    function::{Gt, Neg},
    typeclass::arrow::{First, Firsted},
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
pub trait Subtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

pub fn subtraction() -> OpChain<LiftAdtF, SubtractionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Subtraction<R> for Then<A, B> | Combine<A, B, C> {
        type Subtraction = Combine<Self, R, SubtractionS>;

        fn subtraction(self, rhs: R) -> Self::Subtraction {
            Combine(self, rhs, SubtractionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubtractionS;

impl LiftEvaluate<Dist<f32>> for SubtractionS {
    type LiftEvaluate = ComposeT<
        BooleanConditional<
            Composed<Gt, Firsted<Neg>>,
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
                Gt.compose(Neg.first()),
                CopyContext::default(),
                CopyContext::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for SubtractionS
where
    D: Pair,
{
    type LiftEvaluate = ComposeT<
        BooleanConditional<
            Composed<Gt, Firsted<Neg>>,
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
                Gt.compose(Neg.first()),
                EvaluateSide::<Left, Inherited, ContextOut>::default(),
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}
