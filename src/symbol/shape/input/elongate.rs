use crate::{EvaluateFunction, EvaluateInputs, IntoMonad, LiftAdt, Modify, Position};

use rust_gpu_bridge::{Sign, Abs};
use crate::glam::Vec2;
use t_funk::{
    closure::{Closure, Curry2, Curry2A, OutputT},
    typeclass::{functor::Fmap, monad::Identity}, macros::lift,
};

// Reflect input modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Elongate<T>(pub T, pub bool);

impl<T, F> Fmap<F> for Elongate<T>
where
    F: Closure<T>,
{
    type Fmap = Elongate<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Elongate(f.call(self.0), self.1)
    }
}

impl<T> IntoMonad for Elongate<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Elongate<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for Elongate<T>
where
    Self: EvaluateFunction<D>,
{
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl<D> EvaluateFunction<D> for Elongate<Vec2> {
    type Function = Curry2A<ElongateF, (Vec2, bool)>;

    fn evaluate_function(self) -> Self::Function {
        ElongateF.prefix2((self.0, self.1))
    }
}

#[lift]
pub fn elongate_f((vector, abs): (Vec2, bool), Position(p): Position<Vec2>) -> Position<Vec2> {
    let l = vector.length();
    let n = vector.normalize();
    let dp = n.dot(p);
    let ds = dp.sign();
    let da = if abs { dp.abs() } else { dp };
    let d = da.min(l) * ds;

    Position(p - n * d)
}
