use std::ops::BitXor;

use crate::{Combine, Elysian, Field, OuterBound, Sequence};

impl<T, R> BitXor<R> for Field<T> {
    type Output = Combine<Self, R, OuterBound>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Combine(self, rhs, OuterBound)
    }
}

impl<T, N, R> BitXor<R> for Sequence<T, N> {
    type Output = Combine<Self, R, OuterBound>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Combine(self, rhs, OuterBound)
    }
}

impl<L1, L2, O, R> BitXor<R> for Combine<L1, L2, O> {
    type Output = Combine<Self, R, OuterBound>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Combine(self, rhs, OuterBound)
    }
}

pub trait CombineOuterBound<T> {
    type OuterBound;

    fn outer_bound(self, t: T) -> Self::OuterBound;
}

impl<T, U> CombineOuterBound<U> for T
where
    T: Elysian + BitXor<U>,
{
    type OuterBound = T::Output;

    fn outer_bound(self, t: U) -> Self::OuterBound {
        self.bitxor(t)
    }
}
