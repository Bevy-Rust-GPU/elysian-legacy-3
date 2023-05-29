mod field;
mod input;
mod output;

pub use field::*;
pub use input::*;
pub use output::*;

use crate::{Domain, DomainF};

use type_fields::{
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
    t_funk::{
        hlist::{Nil, PushBack, Cons},
        list::{
            hlist::{Chain, ChainT},
            tlist::ToHList,
        },
        Fmap, FmapT,
    },
};

use core::ops::Shl;
use std::fmt::Debug;

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Shape<T>(pub T);

impl<T, U> Domain<U> for Shape<T>
where
    T: Fmap<DomainF<U>>,
    FmapT<T, DomainF<U>>: Chain,
{
    type Domain = ChainT<FmapT<T, DomainF<U>>>;

    fn domain(self) -> Self::Domain {
        self.0.fmap(DomainF::default()).chain()
    }
}

// Composition
impl<T, U> Shl<U> for Shape<T>
where
    T: PushBack<U>,
{
    type Output = Shape<T::PushBack>;

    fn shl(self, rhs: U) -> Self::Output {
        Shape(self.0.push_back(rhs))
    }
}

pub fn shape() -> Shape<Nil> {
    Shape(Nil)
}


/// A shape whose first instruction is Lift,
/// thus guaranteeing that it is the root shape
pub type RootShape<T> = Shape<Cons<Lift, Cons<T, Nil>>>;

pub fn root_shape() -> Shape<Cons<Lift, Nil>> {
    Shape(Cons(Lift, Nil))
}

pub trait ToShape: Sized {
    type ToShape;

    fn shape(self) -> Shape<Self::ToShape>;
}

impl<T> ToShape for T
where
    T: ToHList,
{
    type ToShape = T::HList;
    fn shape(self) -> Shape<Self::ToShape> {
        Shape(self.to_hlist())
    }
}
