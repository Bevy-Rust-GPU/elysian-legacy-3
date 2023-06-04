use std::ops::Mul;

use crate::{Combine, Elysian, Field, Intersection, Sequence};

impl<T, R> Mul<R> for Field<T> {
    type Output = Combine<Self, R, Intersection>;

    fn mul(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Intersection)
    }
}

impl<T, N, R> Mul<R> for Sequence<T, N> {
    type Output = Combine<Self, R, Intersection>;

    fn mul(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Intersection)
    }
}

impl<L1, L2, O, R> Mul<R> for Combine<L1, L2, O> {
    type Output = Combine<Self, R, Intersection>;

    fn mul(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Intersection)
    }
}

pub trait CombineIntersection<T> {
    type Intersection;

    fn intersection(self, t: T) -> Self::Intersection;
}

impl<T, U> CombineIntersection<U> for T
where
    T: Elysian + Mul<U>,
{
    type Intersection = T::Output;

    fn intersection(self, t: U) -> Self::Intersection {
        self.mul(t)
    }
}
