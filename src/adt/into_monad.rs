//! Lift a concrete value into a monadic value

use t_funk::{
    collection::hlist::{Cons, Nil},
    macros::{functions, impl_adt, types},
    typeclass::monad::Identity,
};

use crate::{Alias, Combine, Domains, Modify, Run};

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

impl<A, B, C, D, E, F, G, H> IntoMonad for (A, B, C, D, E, F, G, H) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G, H, I> IntoMonad for (A, B, C, D, E, F, G, H, I) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G, H, I, J> IntoMonad for (A, B, C, D, E, F, G, H, I, J) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> IntoMonad for (A, B, C, D, E, F, G, H, I, J, K) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> IntoMonad for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M> IntoMonad for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<A, B> IntoMonad for Cons<A, B> {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl IntoMonad for Nil {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl_adt! {
    impl<A, B, C> IntoMonad for Run<A> | Modify<A> | Domains<A> | Alias<A> | Combine<A, B, C> {
        type IntoMonad = Identity<Self>;

        fn into_monad(self) -> Self::IntoMonad {
            Identity(self)
        }
    }
}
