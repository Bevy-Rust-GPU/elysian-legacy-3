//! Lift a concrete value into a monadic value

use t_funk::{
    macros::{functions, types},
    typeclass::monad::Identity,
};

use crate::{Alias, Combine, Run};

#[functions]
#[types]
pub trait IntoMonad {
    type IntoMonad;

    fn into_monad(self) -> Self::IntoMonad;
}

impl<T> IntoMonad for Identity<T> {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A> IntoMonad for (A,) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B> IntoMonad for (A, B) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C> IntoMonad for (A, B, C) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D> IntoMonad for (A, B, C, D) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E> IntoMonad for (A, B, C, D, E) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F> IntoMonad for (A, B, C, D, E, F) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G> IntoMonad for (A, B, C, D, E, F, G) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C> IntoMonad for Combine<A, B, C> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> IntoMonad for Run<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> IntoMonad for Alias<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}
