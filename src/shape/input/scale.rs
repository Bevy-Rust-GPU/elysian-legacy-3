use crate::{impl_identity, impl_split, impl_subtree, DistanceF32, Domain, GradientF32, Position, impl_null, PositionF32, impl_domains};

use type_fields::{
    macros::{
        applicative::Applicative, arrow::Arrow, category::Category, functor::Functor, monad::Monad,
    },
    t_funk::Closure,
};

// Scale input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
pub struct Scale<T>(pub T);

impl<T> Domain<DistanceF32> for Scale<T> {
    type Input = PositionF32;
    type Domain = ScaleF<T>;

    fn domain(self) -> Self::Domain {
        ScaleF(self.0)
    }
}

impl<T> Domain<GradientF32> for Scale<T> {
    type Input = PositionF32;
    type Domain = ScaleF<T>;

    fn domain(self) -> Self::Domain {
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

impl_identity!(Scale<T>);
impl_null!(Scale<T>);
impl_domains!(Scale<T>);
impl_split!(Scale<T>);
impl_subtree!(Scale<T>);
