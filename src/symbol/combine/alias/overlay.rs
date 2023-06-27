use t_funk::{
    function::Lt,
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Distance, EvaluateSide,
    ExpandAlias, Inherited, InsertProperty, IntoMonad, IntoMonadT, Left, LiftAdt, Right,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait Overlay<T> {
    type Overlay;

    fn overlay(self, rhs: T) -> Self::Overlay;
}

impl<T, U> Overlay<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type Overlay = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<OverlayS>>;

    fn overlay(self, rhs: U) -> Self::Overlay {
        Combine(self.into_monad(), rhs.into_monad(), OverlayS.into_monad())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OverlayS;

impl<F> Fmap<F> for OverlayS {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for OverlayS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for OverlayS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for OverlayS {
    type ExpandAlias = (
        EvaluateSide<Right, Inherited, ContextA>,
        CopyContext<ContextA, ContextB>,
        InsertProperty<Distance<f32>, ContextB>,
        BooleanConditional<
            Lt,
            CopyContext<ContextA, ContextOut>,
            EvaluateSide<Left, Inherited, ContextOut>,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
