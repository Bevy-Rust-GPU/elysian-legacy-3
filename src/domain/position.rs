//! Position domain
//!
//! Mostly implicit, since the position domain acts as input

use crate::LiftParam;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T>(pub T);

impl<T, D> LiftParam<D> for Position<T> {
    type LiftParam = Self;

    fn lift_param(self, _: D) -> Self::LiftParam {
        self
    }
}
