use std::{marker::PhantomData, ops::Add};

use type_fields::t_funk::Lt;

use crate::Shape;

use super::Boolean;

pub type Unioned<T, U> = Boolean<T, U, Lt>;

// \/
pub trait Union<T>: Sized {
    fn union(self, t: T) -> Unioned<Self, T> {
        Boolean(self, t, PhantomData)
    }
}

impl<L, R> Union<R> for L {}

impl<L1, L2, O, R> Add<R> for Boolean<L1, L2, O> {
    type Output = Unioned<Self, R>;

    fn add(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}

impl<L, R> Add<R> for Shape<L> {
    type Output = Unioned<Self, R>;

    fn add(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}
