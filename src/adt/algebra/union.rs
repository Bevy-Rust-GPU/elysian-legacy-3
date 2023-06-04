use std::ops::Add;

use crate::{Combine, Elysian, Field, Sequence, Union};

impl<T, R> Add<R> for Field<T> {
    type Output = Combine<Self, R, Union>;

    fn add(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Union)
    }
}

impl<T, N, R> Add<R> for Sequence<T, N> {
    type Output = Combine<Self, R, Union>;

    fn add(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Union)
    }
}

impl<L1, L2, O, R> Add<R> for Combine<L1, L2, O> {
    type Output = Combine<Self, R, Union>;

    fn add(self, rhs: R) -> Self::Output {
        Combine(self, rhs, Union)
    }
}

pub trait CombineUnion<T> {
    type CombineUnion;

    fn union(self, t: T) -> Self::CombineUnion;
}

impl<T, U> CombineUnion<U> for T
where
    T: Elysian + Add<U>,
{
    type CombineUnion = T::Output;

    fn union(self, t: U) -> Self::CombineUnion {
        self.add(t)
    }
}
