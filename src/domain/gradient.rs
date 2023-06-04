//! Gradient domain

use crate::{DomainF, DomainFunction, Domain};

// Gradient domain
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<T>(pub T, pub T);
pub type GradientF32 = Gradient<f32>;

pub type GradientT<T> = <T as DomainFunction<Gradient<f32>>>::Function;
pub type GradientF = DomainF<Gradient<f32>>;

impl<T> Domain<Self> for Gradient<T> {
    type Outputs = Self;
}
