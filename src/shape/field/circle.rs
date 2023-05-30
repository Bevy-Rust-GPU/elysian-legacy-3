use crate::{
    DistanceF, DistanceF32, DistanceT, Domain, DomainF, DomainT, GradientF32, Isosurface, Point, impl_subtree, impl_split, impl_identity, impl_null, PositionF32, impl_domains,
};

use type_fields::{
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
    t_funk::{closure::Compose, Closure, Composed},
};

// Point field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Circle<T>(pub T);

impl<T> Domain<DistanceF32> for Circle<T> {
    type Input = PositionF32;
    type Domain = Composed<DistanceT<Isosurface<T>>, DistanceT<Point>>;

    fn domain(self) -> Self::Domain {
        DistanceF::default()
            .call(Point)
            .compose_l(DistanceF::default().call(Isosurface(self.0)))
    }
}

impl<T> Domain<GradientF32> for Circle<T> {
    type Input = PositionF32;
    type Domain = DomainT<Point, GradientF32>;

    fn domain(self) -> Self::Domain {
        DomainF::<GradientF32>::default().call(Point)
    }
}

impl_identity!(Circle<T>);
impl_domains!(Circle<T>);
impl_null!(Circle<T>);
impl_split!(Circle<T>);
impl_subtree!(Circle<T>);
