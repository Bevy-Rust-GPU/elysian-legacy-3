use std::marker::PhantomData;

use t_funk::typeclass::{functor::Fmap, monad::Identity};

use crate::{
    Alias, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, EvaluateSide,
    ExpandAlias, Inherited, IntoMonad, Left, LiftAdt, Right,
};

use t_funk::macros::types;

use crate::Combine;

#[types]
pub trait Replace<U> {
    type Replace<T>;

    fn replace<T>(self, rhs: U) -> Self::Replace<T>;
}

impl<T, R> Replace<R> for T {
    type Replace<U> = Combine<Self, R, Identity<ReplaceS<U>>>;

    fn replace<U>(self, rhs: R) -> Self::Replace<U> {
        Combine(self, rhs, Identity(ReplaceS(PhantomData::<U>)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReplaceS<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for ReplaceS<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for ReplaceS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for ReplaceS<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for ReplaceS<T> {
    type ExpandAlias = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        CopyContext<ContextA, ContextOut>,
        CopyProperty<T, ContextB, ContextOut>,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

