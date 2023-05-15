//! Distance domain
//! Subdomain of position

use std::ops::Neg;

use type_fields::{
    macros::{applicative::Applicative, functor::Functor, monad::Monad, Copointed, Pointed},
    t_funk::{
        hlist::{Chain, ChainT},
        Either, Fmap, FmapT,
    },
};

use crate::{Domain, DomainF, DomainT};

// Distance domain values
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Functor,
    Applicative,
    Monad,
)]
pub struct Distance<T>(pub T);

impl<T> Neg for Distance<T>
where
    T: Neg,
{
    type Output = Distance<T::Output>;

    fn neg(self) -> Self::Output {
        Distance(self.0.neg())
    }
}

pub type DistanceF32 = Distance<f32>;

impl<L, R> Domain<DistanceF32> for Either<L, R>
where
    L: Fmap<DistanceF>,
    FmapT<L, DistanceF>: Chain,
    R: Fmap<DistanceF>,
    FmapT<R, DistanceF>: Chain,
{
    type Domain = Either<ChainT<FmapT<L, DistanceF>>, ChainT<FmapT<R, DistanceF>>>;

    fn domain(self) -> Self::Domain {
        match self {
            Either::Left(l) => Either::Left(l.fmap(DistanceF::default()).chain()),
            Either::Right(r) => Either::Right(r.fmap(DistanceF::default()).chain()),
        }
    }
}

pub type DistanceT<T> = DomainT<T, Distance<f32>>;
pub type DistanceF = DomainF<Distance<f32>>;
