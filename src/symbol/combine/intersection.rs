use t_funk::{function::Gt, typeclass::monad::Identity};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, IntoMonad, IntoMonadT, Left, LiftEvaluate, Pair, Right,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

impl<T, U> Intersection<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type Intersection = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<IntersectionS>>;

    fn intersection(self, rhs: U) -> Self::Intersection {
        Combine(
            self.into_monad(),
            rhs.into_monad(),
            IntersectionS.into_monad(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntersectionS;

impl IntoMonad for IntersectionS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftEvaluate<Dist<f32>> for IntersectionS {
    type LiftEvaluate = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        BooleanConditional<
            Gt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for IntersectionS
where
    D: Pair,
{
    type LiftEvaluate = (
        EvaluateSide<Left, Dist<f32>, ContextA>,
        EvaluateSide<Right, Dist<f32>, ContextB>,
        BooleanConditional<
            Gt,
            EvaluateSide<Left, Inherited, ContextOut>,
            EvaluateSide<Right, Inherited, ContextOut>,
            Distance<f32>,
        >,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
