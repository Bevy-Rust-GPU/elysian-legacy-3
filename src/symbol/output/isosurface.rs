use core::ops::Sub;

use crate::{Distance, DistanceF32, DomainFunction, GradientF32, LiftAdt, Output};

use t_funk::{
    closure::Closure,
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
};

// Isosurface output modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Isosurface<T>(pub T);

impl<T> LiftAdt for Isosurface<T> {
    type LiftAdt = Output<Self>;

    fn adt(self) -> Self::LiftAdt {
        Output(self)
    }
}

impl<T> DomainFunction<DistanceF32> for Isosurface<T> {
    type Inputs = DistanceF32;
    type Function = IsosurfaceDistance<T>;

    fn domain(self) -> Self::Function {
        IsosurfaceDistance(self.0)
    }
}

impl<T> DomainFunction<GradientF32> for Isosurface<T> {
    type Inputs = ();
    type Function = ();

    fn domain(self) -> Self::Function {
        ()
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
