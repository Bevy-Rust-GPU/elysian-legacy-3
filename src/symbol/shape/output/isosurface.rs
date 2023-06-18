use core::ops::Sub;

use crate::{Distance, Evaluable, EvaluateFunction, LiftAdt, LiftModify, Run};

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
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T> Evaluable for Isosurface<T> {
    type Evaluable = LiftModify;
}

impl<T, D> EvaluateFunction<D> for Isosurface<T> {
    type Inputs = Distance<f32>;
    type Moves = ();
    type Function = Curry2B<IsosurfaceDistance, T>;

    fn evaluate_function(self) -> Self::Function {
        IsosurfaceDistance.suffix2(self.0)
    }
}

#[lift]
pub fn isosurface_distance<T>(Distance(input): Distance<T>, iso: T) -> Distance<T::Output>
where
    T: Sub<T>,
{
    Distance(input - iso)
}
