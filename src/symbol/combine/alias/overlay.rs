use t_funk::{
    closure::Curry2B,
    function::Lt,
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, ContextOut, Dist, Distance, EvaluateSide, ExpandAlias, Inherited, IntoMonad, IntoTuple,
    IntoTupleT, Left, LiftAdt, Right, UnaryConditional,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait MakeOverlay<T> {
    type Overlay;

    fn overlay(self, rhs: T) -> Self::Overlay;
}

pub fn overlay() -> Overlay {
    Overlay
}

impl<T, U> MakeOverlay<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type Overlay = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<Overlay>>;

    fn overlay(self, rhs: U) -> Self::Overlay {
        Combine(self.into_tuple(), rhs.into_tuple(), Overlay.into_tuple())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Overlay;

impl<F> Fmap<F> for Overlay {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for Overlay {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Overlay {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for Overlay {
    type ExpandAlias = (
        EvaluateSide<Right, Dist<f32>, ContextOut>,
        UnaryConditional<
            ContextOut,
            Distance<f32>,
            Curry2B<Lt, Distance<f32>>,
            EvaluateSide<Right, Inherited, ContextOut>,
            EvaluateSide<Left, Inherited, ContextOut>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
