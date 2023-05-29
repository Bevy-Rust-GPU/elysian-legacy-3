//! Passthrough application
//!
//! Break the current value out of a shape composition chain
//! and feed it into an intermediary computation
//! without affecting the final return value.
//!
//! For example, this can be used to display intermediary
//! shapes independently of their final composite.

use std::ops::Shr;

use type_fields::t_funk::Closure;

use crate::{Boolean, Shape, Bounding};

impl<T, U> Shr<U> for Shape<T>
where
    T: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shr(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<A, B, O, U> Shr<U> for Boolean<A, B, O>
where
    A: Clone,
    B: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shr(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<A, B, O, U> Shr<U> for Bounding<A, B, O>
where
    A: Clone,
    B: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shr(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}
