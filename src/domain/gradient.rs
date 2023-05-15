//! Gradient domain

use type_fields::t_funk::{
    hlist::{Chain, ChainT},
    Either, Fmap, FmapT,
};

use crate::{Domain, DomainF};

// Gradient domain
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<T>(pub T, pub T);
pub type GradientF32 = Gradient<f32>;

pub type GradientT<T> = <T as Domain<Gradient<f32>>>::Domain;
pub type GradientF = DomainF<Gradient<f32>>;

impl<L, R> Domain<GradientF32> for Either<L, R>
where
    L: Fmap<GradientF>,
    FmapT<L, GradientF>: Chain,
    R: Fmap<GradientF>,
    FmapT<R, GradientF>: Chain,
{
    type Domain = Either<ChainT<FmapT<L, GradientF>>, ChainT<FmapT<R, GradientF>>>;

    fn domain(self) -> Self::Domain {
        match self {
            Either::Left(l) => Either::Left(l.fmap(DomainF::default()).chain()),
            Either::Right(r) => Either::Right(r.fmap(DomainF::default()).chain()),
        }
    }
}
