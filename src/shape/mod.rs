mod field;
mod input;
mod output;

pub use field::*;
pub use input::*;
pub use output::*;

use crate::{Domain, Evaluate, EvaluateF};

use type_fields::{
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
    t_funk::{
        hlist::{Nil, PushBack},
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

impl<T> Domain<Evaluate> for Shape<T>
where
    T: Fmap<EvaluateF>,
    FmapT<T, EvaluateF>: Chain,
{
    type Domain = ChainT<FmapT<T, EvaluateF>>;

    fn domain(self) -> Self::Domain {
        self.0.fmap(EvaluateF::default()).chain()
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
