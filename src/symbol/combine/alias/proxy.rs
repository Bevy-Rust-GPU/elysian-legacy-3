use std::marker::PhantomData;

use t_funk::{
    function::{Id, Lt},
    typeclass::{monad::Identity, functor::Fmap},
};

use crate::{
    Alias, BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, Distance,
    EvaluateSide, ExpandAlias, Inherited, Left, LiftAdt, Right, IntoMonad,
};

use t_funk::macros::types;

use crate::Combine;

#[types]
pub trait Proxy<U> {
    type Proxy<T>;

    fn proxy<T>(self, rhs: U) -> Self::Proxy<T>;
}

impl<T, R> Proxy<R> for T {
    type Proxy<U> = Combine<Self, R, Identity<ProxyS<U>>>;

    fn proxy<U>(self, rhs: R) -> Self::Proxy<U> {
        Combine(self, rhs, Identity(ProxyS(PhantomData::<U>)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProxyS<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for ProxyS<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for ProxyS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for ProxyS<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for ProxyS<T> {
    type ExpandAlias = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        CopyContext<ContextA, ContextOut>,
        BooleanConditional<Lt, Id, CopyProperty<T, ContextB, ContextOut>, Distance<f32>>,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
