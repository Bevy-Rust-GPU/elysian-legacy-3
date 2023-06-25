use t_funk::{
    closure::Composed,
    function::{Gt, Neg},
    typeclass::{arrow::Firsted, monad::Identity},
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, IntoMonad, IntoMonadT, Left, LiftEvaluate, Pair, Right,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait Subtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

impl<T, U> Subtraction<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type Subtraction = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<SubtractionS>>;

    fn subtraction(self, rhs: U) -> Self::Subtraction {
        Combine(
            self.into_monad(),
            rhs.into_monad(),
            SubtractionS.into_monad(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubtractionS;

impl IntoMonad for SubtractionS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

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
