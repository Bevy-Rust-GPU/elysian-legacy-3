use t_funk::{
    closure::Composed,
    function::{Gt, Neg},
    typeclass::{arrow::Firsted, monad::Identity},
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
        type Subtraction = Combine<Self, R, Identity<SubtractionS>>;

        fn subtraction(self, rhs: R) -> Self::Subtraction {
            Combine(self, rhs, Identity(SubtractionS))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubtractionS;

impl LiftEvaluate<Dist<f32>> for SubtractionS {
    type LiftEvaluate = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        BooleanConditional<
            Composed<Gt, Firsted<Neg>>,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for SubtractionS
where
    D: Pair,
{
    type LiftEvaluate = (
        EvaluateSide<Left, Dist<f32>, ContextA>,
        EvaluateSide<Right, Dist<f32>, ContextB>,
        BooleanConditional<
            Composed<Gt, Firsted<Neg>>,
            EvaluateSide<Left, Inherited, ContextOut>,
            EvaluateSide<Right, Inherited, ContextOut>,
            Distance<f32>,
        >,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
