use std::ops::BitAnd;

use crate::{Combine, Elysian, Field, InnerBound, Sequence};

impl<T, R> BitAnd<R> for Field<T> {
    type Output = Combine<Self, R, InnerBound>;

    fn bitand(self, rhs: R) -> Self::Output {
        Combine(self, rhs, InnerBound)
    }
}

impl<T, N, R> BitAnd<R> for Sequence<T, N> {
    type Output = Combine<Self, R, InnerBound>;

    fn bitand(self, rhs: R) -> Self::Output {
        Combine(self, rhs, InnerBound)
    }
}

impl<L1, L2, O, R> BitAnd<R> for Combine<L1, L2, O> {
    type Output = Combine<Self, R, InnerBound>;

    fn bitand(self, rhs: R) -> Self::Output {
        Combine(self, rhs, InnerBound)
    }
}

pub trait CombineInnerBound<T> {
    type CombineInnerBound;

    fn inner_bound(self, t: T) -> Self::CombineInnerBound;
}

impl<T, U> CombineInnerBound<U> for T
where
    T: Elysian + BitAnd<U>,
{
    type CombineInnerBound = T::Output;

    fn inner_bound(self, t: U) -> Self::CombineInnerBound {
        self.bitand(t)
    }
}
