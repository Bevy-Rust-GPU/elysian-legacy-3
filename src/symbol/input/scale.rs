use crate::{DistanceF32, DomainFunction, GradientF32, Input, LiftAdt, Position, PositionF32};

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
pub struct Scale<T>(pub T);

impl<T> LiftAdt for Scale<T> {
    type LiftAdt = Input<Self>;

    fn adt(self) -> Self::LiftAdt {
        Input(self)
    }
}

impl<T> DomainFunction<DistanceF32> for Scale<T> {
    type Inputs = PositionF32;
    type Function = ScaleF<T>;

    fn domain(self) -> Self::Function {
        ScaleF(self.0)
    }
}

impl<T> DomainFunction<GradientF32> for Scale<T> {
    type Inputs = PositionF32;
    type Function = ScaleF<T>;

    fn domain(self) -> Self::Function {
        ScaleF(self.0)
    }
}

// General scale function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct ScaleF<T>(T);

impl<T> Closure<Position<T>> for ScaleF<T>
where
    T: Clone + core::ops::Div<Output = T>,
{
    type Output = Position<T>;

    fn call(self, Position(x, y): Position<T>) -> Self::Output {
        let s = self.0;
        Position(x / s.clone(), y / s)
    }
}
