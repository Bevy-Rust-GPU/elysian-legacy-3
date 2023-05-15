use crate::{DistanceF32, Domain, GradientF32, Position};

use type_fields::{
    macros::{arrow::Arrow, category::Category},
    t_funk::{
        arrow::{First, FirstT},
        Closure,
    },
};

// Translation input modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Translate<T>(pub T, pub T);

impl<T> Domain<DistanceF32> for Translate<T> {
    type Domain = TranslateF<T>;

    fn domain(self) -> Self::Domain {
        TranslateF(self.0, self.1)
    }
}

impl<T> Domain<GradientF32> for Translate<T> {
    type Domain = FirstT<TranslateF<T>>;

    fn domain(self) -> Self::Domain {
        TranslateF(self.0, self.1).first()
    }
}

// General translation function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Category, Arrow)]
pub struct TranslateF<T>(T, T);

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
