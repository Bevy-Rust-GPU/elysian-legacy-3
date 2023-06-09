use crate::{Distance, DomainFunction, Gradient, Input, LiftAdt, Position, ShapeEnd};

use glam::Vec2;
use t_funk::{
    closure::Closure,
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
};

// Translation input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translate<T>(pub T);

impl<T> LiftAdt for Translate<T> {
    type LiftAdt = Input<Self, ShapeEnd>;

    fn lift_adt(self) -> Self::LiftAdt {
        Input(self, ShapeEnd)
    }
}

impl<T> DomainFunction<Distance<f32>> for Translate<T> {
    type Inputs = Position<Vec2>;
    type Function = TranslateF<T>;

    fn domain(self) -> Self::Function {
        TranslateF(self.0)
    }
}

impl<T> DomainFunction<Gradient<Vec2>> for Translate<T> {
    type Inputs = Position<Vec2>;
    type Function = TranslateF<T>;

    fn domain(self) -> Self::Function {
        TranslateF(self.0)
    }
}

// General translation function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct TranslateF<T>(pub T);

impl<P> Closure<Position<P>> for TranslateF<P>
where
    P: core::ops::Sub<Output = P>,
{
    type Output = Position<P>;

    fn call(self, Position(p): Position<P>) -> Self::Output {
        Position(p - self.0)
    }
}
