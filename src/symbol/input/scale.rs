use crate::{Distance, DomainFunction, Gradient, Input, LiftAdt, Position};

use glam::Vec2;
use t_funk::{
    closure::Closure,
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
};

// Scale input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale<S>(pub S);

impl<S> LiftAdt for Scale<S> {
    type LiftAdt = Input<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Input(self)
    }
}

impl<S> DomainFunction<Distance<f32>> for Scale<S> {
    type Inputs = Position<Vec2>;
    type Function = ScaleF<S>;

    fn domain(self) -> Self::Function {
        ScaleF(self.0)
    }
}

impl<S> DomainFunction<Gradient<Vec2>> for Scale<S> {
    type Inputs = Position<Vec2>;
    type Function = ScaleF<S>;

    fn domain(self) -> Self::Function {
        ScaleF(self.0)
    }
}

// General scale function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct ScaleF<T>(T);

impl<P, S> Closure<Position<P>> for ScaleF<S>
where
    P: Clone + core::ops::Div<S, Output = P>,
{
    type Output = Position<P>;

    fn call(self, Position(p): Position<P>) -> Self::Output {
        let s = self.0;
        Position(p / s)
    }
}
