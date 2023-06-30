//! Color domain

use crate::glam::Vec4;

use crate::{LiftParam, Set, SetT};

pub trait SetColor<T> {
    type SetColor;

    fn color(self, color: T) -> Self::SetColor;
}

impl<T, U> SetColor<U> for T
where
    T: Set<Color<U>>,
{
    type SetColor = SetT<T, Color<U>>;

    fn color(self, color: U) -> Self::SetColor {
        self.set(Color(color))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<T>(pub T);

impl<T, D> LiftParam<D> for Color<T> {
    type LiftParam = Self;

    fn lift_param(self, _: D) -> Self::LiftParam {
        self
    }
}

pub const YELLOW: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
pub const CYAN: Vec4 = Vec4::new(0.0, 1.0, 1.0, 1.0);
pub const WHITE: Vec4 = Vec4::ONE;
pub const BLACK: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
pub const TRANSPARENT: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.0);
