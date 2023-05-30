use crate::{
    impl_domains, impl_identity, impl_null, impl_split, impl_subtree, DistanceF32, Domain,
    GradientF32, Position, PositionF32,
};

use type_fields::{
    macros::{arrow::Arrow, category::Category},
    t_funk::Closure,
};

// Translation input modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Translate<T>(pub T, pub T);

impl<T> Domain<DistanceF32> for Translate<T> {
    type Input = PositionF32;
    type Domain = TranslateF<T>;

    fn domain(self) -> Self::Domain {
        TranslateF(self.0, self.1)
    }
}

impl<T> Domain<GradientF32> for Translate<T> {
    type Input = PositionF32;
    type Domain = TranslateF<T>;

    fn domain(self) -> Self::Domain {
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

impl_identity!(Translate<T>);
impl_domains!(Translate<T>);
impl_null!(Translate<T>);
impl_split!(Translate<T>);
impl_subtree!(Translate<T>);
