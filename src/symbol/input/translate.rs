use crate::{
    DistanceF32, DomainFunction, GradientF32, Input, LiftShape, Position, PositionF32,
};

use t_funk::{
    closure::{Closure, OutputT},
    macros::{arrow::Arrow, category::Category},
    typeclass::functor::Fmap,
};

// Translation input modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translate<T>(pub T, pub T);

impl<T, F> Fmap<F> for Translate<T>
where
    F: Clone + Closure<T>,
{
    type Fmap = Translate<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Translate(f.clone().call(self.0), f.call(self.1))
    }
}

impl<T> LiftShape for Translate<T> {
    type LiftShape = Input<Self>;

    fn lift_shape(self) -> Self::LiftShape {
        Input(self)
    }
}

impl<T> DomainFunction<DistanceF32> for Translate<T> {
    type Inputs = PositionF32;
    type Function = TranslateF<T>;

    fn domain(self) -> Self::Function {
        TranslateF(self.0, self.1)
    }
}

impl<T> DomainFunction<GradientF32> for Translate<T> {
    type Inputs = PositionF32;
    type Function = TranslateF<T>;

    fn domain(self) -> Self::Function {
        TranslateF(self.0, self.1)
    }
}

// General translation function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct TranslateF<T>(pub T, pub T);

impl<T> Closure<Position<T>> for TranslateF<T>
where
    T: core::ops::Sub<Output = T>,
{
    type Output = Position<T>;

    fn call(self, Position(x, y): Position<T>) -> Self::Output {
        let TranslateF(dx, dy) = self;
        Position(x - dx, y - dy)
    }
}
