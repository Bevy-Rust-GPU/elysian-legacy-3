use core::marker::PhantomData;

use t_funk::{
    function::{Id, Lt},
    typeclass::{functor::Fmap, monad::Identity, semigroup::MappendT},
};

use crate::{
    Alias, BinaryConditional, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, Distance,
    EvaluateBoth, ExpandAlias, ExpandAliasT, Inherited, IntoMonad, LiftAdt,
};

use t_funk::macros::types;

use crate::Combine;

#[types]
pub trait MakeProxy<U> {
    type Proxy<T>;

    fn proxy<T>(self, rhs: U) -> Self::Proxy<T>;
}

impl<T, R> MakeProxy<R> for T {
    type Proxy<U> = Combine<Self, R, Identity<Proxy<U>>>;

    fn proxy<U>(self, rhs: R) -> Self::Proxy<U> {
        Combine(self, rhs, Identity(Proxy(PhantomData::<U>)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proxy<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for Proxy<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for Proxy<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Proxy<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for Proxy<T> {
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<Inherited>, D>,
        (
            CopyContext<ContextA, ContextOut>,
            BinaryConditional<Distance<f32>, Lt, Id, CopyProperty<T, ContextB, ContextOut>>,
        ),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
