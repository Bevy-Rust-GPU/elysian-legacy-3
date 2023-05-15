use crate::{Distance, DistanceF32, Domain, Gradient, GradientF32, Position, PositionF32};

use type_fields::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::{
        arrow::{First, Firsted},
        Function, FlipTuple, closure::Compose, Composed,
    },
};

// Point field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point;

// Distance
impl Domain<DistanceF32> for Point {
    type Domain = PointDistance;

    fn domain(self) -> Self::Domain {
        PointDistance
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointDistance;

impl Function<PositionF32> for PointDistance {
    type Output = DistanceF32;

    fn call(Position(x, y): PositionF32) -> Self::Output {
        Distance((x * x + y * y).sqrt())
    }
}

// Gradient
impl Domain<GradientF32> for Point {
    type Domain = Composed<FlipTuple, Firsted<PointGradient>>;

    fn domain(self) -> Self::Domain {
        PointGradient.first().compose_l(FlipTuple)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointGradient;

impl Function<Position<f32>> for PointGradient {
    type Output = GradientF32;

    fn call(Position(x, y): Position<f32>) -> Self::Output {
        let l = (x * x + y * y).sqrt();
        Gradient(x / l, y / l)
    }
}
