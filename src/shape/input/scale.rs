use crate::{DistanceF32, Domain, GradientF32, Position};

use type_fields::{
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
    t_funk::{arrow::First, Closure, FirstT},
};

// Scale input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Scale<T>(pub T);

impl<T> Domain<DistanceF32> for Scale<T> {
    type Domain = ScaleF<T>;

    fn domain(self) -> Self::Domain {
        ScaleF(self.0)
    }
}

impl<T> Domain<GradientF32> for Scale<T> {
    type Domain = FirstT<ScaleF<T>>;

    fn domain(self) -> Self::Domain {
        ScaleF(self.0).first()
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
