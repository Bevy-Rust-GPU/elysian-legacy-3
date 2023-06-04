use std::ops::Sub;

use crate::{Combine, Elysian, Field, Sequence, Subtraction};

impl<T, R> Sub<R> for Field<T> {
    type Output = Combine<Self, R, Subtraction>;

    fn sub(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Subtraction)
    }
}

impl<T, N, R> Sub<R> for Sequence<T, N> {
    type Output = Combine<Self, R, Subtraction>;

    fn sub(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Subtraction)
    }
}

impl<L1, L2, O, R> Sub<R> for Combine<L1, L2, O> {
    type Output = Combine<Self, R, Subtraction>;

    fn sub(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Subtraction)
    }
}

pub trait CombineSubtraction<T> {
    type Subtraction;

    fn subtraction(self, t: T) -> Self::Subtraction;
}

impl<T, U> CombineSubtraction<U> for T
where
    T: Elysian + Sub<U>,
{
    type Subtraction = T::Output;

    fn subtraction(self, t: U) -> Self::Subtraction {
        self.sub(t)
    }
}
