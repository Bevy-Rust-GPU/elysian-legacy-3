use core::ops::Sub;

use crate::{Distance, DomainFunction, Gradient, LiftAdt, Output, ShapeEnd};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
};

// Isosurface output modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Isosurface<T>(pub T);

impl<T> LiftAdt for Isosurface<T> {
    type LiftAdt = Output<Self, ShapeEnd>;

    fn lift_adt(self) -> Self::LiftAdt {
        Output(self, ShapeEnd)
    }
}

impl<T> DomainFunction<Distance<f32>> for Isosurface<T> {
    type Inputs = Distance<f32>;
    type Moves = ();
    type Function = Curry2B<IsosurfaceDistance, T>;

    fn domain(self) -> Self::Function {
        IsosurfaceDistance.suffix2(self.0)
    }
}

impl<T> DomainFunction<Gradient<Vec2>> for Isosurface<T> {
    type Inputs = ();
    type Moves = ();
    type Function = ();

    fn domain(self) -> Self::Function {
        ()
    }
}

#[lift]
pub fn isosurface_distance<T>(Distance(input): Distance<T>, iso: T) -> Distance<T::Output>
where
    T: Sub<T>,
{
    Distance(input - iso)
}
