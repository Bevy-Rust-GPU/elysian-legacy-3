//! Position domain
//!
//! Mostly implicit, since the position domain acts as input

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T>(pub T, pub T);

pub type PositionF32 = Position<f32>;
