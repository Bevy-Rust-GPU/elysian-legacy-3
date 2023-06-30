use core::marker::PhantomData;

use t_funk::typeclass::{functor::Fmap, monad::Identity, semigroup::MappendT};

use crate::{
    Alias, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, EvaluateBoth, ExpandAlias,
    ExpandAliasT, Inherited, IntoMonad, LiftAdt,
};

use t_funk::macros::types;

use crate::Combine;

#[types]
pub trait MakeReplace<U> {
    type Replace<T>;

    fn replace<T>(self, rhs: U) -> Self::Replace<T>;
}

impl<T, R> MakeReplace<R> for T {
    type Replace<U> = Combine<Self, R, Identity<Replace<U>>>;

    fn replace<U>(self, rhs: R) -> Self::Replace<U> {
        Combine(self, rhs, Identity(Replace(PhantomData::<U>)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Replace<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for Replace<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for Replace<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Replace<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for Replace<T> {
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<Inherited>, D>,
        (
            CopyContext<ContextA, ContextOut>,
            CopyProperty<T, ContextB, ContextOut>,
        ),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
