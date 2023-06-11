//! Color domain

use crate::LiftParam;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<T>(pub T);

impl<T, D> LiftParam<D> for Color<T> {
    type LiftParam = Self;

    fn lift_param(self, _: D) -> Self::LiftParam {
        self
    }
}
