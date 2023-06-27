use crate::{
    Distance, Domains, EvaluateFunction, EvaluateInputs, Gradient, IntoMonad, LiftAdt, Position,
};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2A},
    macros::lift,
    typeclass::{functor::Fmap, monad::Identity},
};

// Point field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plane<T>(pub T);

impl<T, F> Fmap<F> for Plane<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for Plane<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Plane<T> {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for Plane<Vec2> {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Distance<f32>> for Plane<Vec2> {
    type Function = Curry2A<PlaneDistance, Vec2>;

    fn evaluate_function(self) -> Self::Function {
        PlaneDistance.prefix2(self.0)
    }
}

impl EvaluateInputs<Gradient<Vec2>> for Plane<Vec2> {
    type Inputs = ();
    type Moves = ();
}

impl EvaluateFunction<Gradient<Vec2>> for Plane<Vec2> {
    type Function = Curry2A<PlaneGradient, Vec2>;

    fn evaluate_function(self) -> Self::Function {
        PlaneGradient.prefix2(self.0)
    }
}

#[lift]
pub fn plane_distance(normal: Vec2, Position(p): Position<Vec2>) -> Distance<f32> {
    Distance(-p.dot(normal))
}

#[lift]
pub fn plane_gradient(normal: Vec2, (): ()) -> Gradient<Vec2> {
    Gradient(normal)
}
