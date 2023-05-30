//! Gradient domain

use crate::{Domain, DomainF};

// Gradient domain
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<T>(pub T, pub T);
pub type GradientF32 = Gradient<f32>;

pub type GradientT<T> = <T as Domain<Gradient<f32>>>::Domain;
pub type GradientF = DomainF<Gradient<f32>>;
