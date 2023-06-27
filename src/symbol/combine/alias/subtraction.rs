use t_funk::{
    closure::ComposeLT,
    function::{Gt, Neg},
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance,
    EvaluateSide, ExpandAlias, Inherited, IntoMonad, IntoMonadT, Left, LiftAdt, MapProperty, Pair,
    Right,
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

impl<F> Fmap<F> for SubtractionS {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for SubtractionS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for SubtractionS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl ExpandAlias<Dist<f32>> for SubtractionS {
    type ExpandAlias = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        MapProperty<ContextB, Distance<f32>, Neg>,
        BooleanConditional<
            Gt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

impl<D> ExpandAlias<(Distance<f32>, D)> for SubtractionS
where
    D: Pair,
{
    type ExpandAlias = (
        EvaluateSide<Left, Dist<f32>, ContextA>,
        EvaluateSide<Right, Dist<f32>, ContextB>,
        MapProperty<ContextB, Distance<f32>, Neg>,
        BooleanConditional<
            Gt,
            EvaluateSide<Left, Inherited, ContextOut>,
            ComposeLT<
                EvaluateSide<Right, Inherited, ContextOut>,
                MapProperty<ContextOut, Distance<f32>, Neg>,
            >,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
