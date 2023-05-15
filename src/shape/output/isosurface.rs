use core::ops::Sub;

use crate::{DistanceF32, Domain, GradientF32, Distance};

use type_fields::{
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
    t_funk::{function::Id, Closure},
};

// Isosurface output modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Isosurface<T>(pub T);

impl<T> Domain<DistanceF32> for Isosurface<T> {
    type Domain = IsosurfaceDistance<T>;

    fn domain(self) -> Self::Domain {
        IsosurfaceDistance(self.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct IsosurfaceDistance<T>(pub T);

impl<T> Closure<Distance<T>> for IsosurfaceDistance<T>
where
    T: Sub<Output = T>,
{
    type Output = Distance<T>;

    fn call(self, Distance(input): Distance<T>) -> Self::Output {
        Distance(input - self.0)
    }
}

impl<T> Domain<GradientF32> for Isosurface<T> {
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

