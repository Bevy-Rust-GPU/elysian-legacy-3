//! Passthrough application
//!
//! Break the current value out of a shape composition chain
//! and feed it into an intermediary computation
//! without affecting the final return value.
//!
//! For example, this can be used to display intermediary
//! shapes independently of their final composite.

use std::ops::Shl;

use t_funk::closure::Closure;

use crate::{Combine, Elysian, Field, Input, Modify, Output, Sequence};

impl<T, U> Shl<U> for Input<T>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<T, U> Shl<U> for Field<T>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<T, U> Shl<U> for Output<T>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<T, U> Shl<U> for Modify<T>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<A, B, U> Shl<U> for Sequence<A, B>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

impl<A, B, F, U> Shl<U> for Combine<A, B, F>
where
    Self: Clone,
    U: Closure<Self>,
{
    type Output = Self;

    fn shl(self, rhs: U) -> Self::Output {
        rhs.call(self.clone());
        self
    }
}

pub trait Tap<T> {
    type Tap;

    fn tap(self, t: T) -> Self::Tap;
}

impl<T, U> Tap<U> for T
where
    T: Elysian + Shl<U>,
{
    type Tap = T::Output;

    fn tap(self, t: U) -> Self::Tap {
        self.shl(t)
    }
}
