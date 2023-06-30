use core::ops::Sub;

use crate::{
    Distance, EvaluateFunction, EvaluateInputs, IntoMonad, IntoTuple, IntoTupleT, LiftAdt, Modify,
};

use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
    typeclass::{
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

pub trait Isosurface<T> {
    type Isosurface;

    fn isosurface(self, t: T) -> Self::Isosurface;
}

impl<T, U> Isosurface<U> for T
where
    T: IntoTuple,
    IsosurfaceS<U>: IntoTuple,
    IntoTupleT<T>: Mappend<IntoTupleT<IsosurfaceS<U>>>,
{
    type Isosurface = MappendT<IntoTupleT<T>, IntoTupleT<IsosurfaceS<U>>>;

    fn isosurface(self, t: U) -> Self::Isosurface {
        self.into_tuple().mappend(IsosurfaceS(t).into_tuple())
    }
}

// Isosurface output modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsosurfaceS<T>(pub T);

impl<T> IntoMonad for IsosurfaceS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for IsosurfaceS<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for IsosurfaceS<T> {
    type Inputs = Distance<f32>;
    type Moves = ();
}

impl<T, D> EvaluateFunction<D> for IsosurfaceS<T> {
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
