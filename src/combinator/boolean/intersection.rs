use std::{marker::PhantomData, ops::Mul};

use crate::Shape;

use super::Boolean;
use type_fields::t_funk::{Composed, Fst, Gt};

pub type Intersect = Composed<Gt, Fst>;
pub type Intersectioned<L, R> = Boolean<L, R, Intersect>;

// /\
pub trait Intersection<T>: Sized {
    fn intersection(self, t: T) -> Intersectioned<Self, T> {
        Boolean(self, t, PhantomData)
    }
}

impl<L, R> Intersection<R> for L {}

impl<L1, L2, O, R> Mul<R> for Boolean<L1, L2, O> {
    type Output = Intersectioned<Self, R>;

    fn mul(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}

impl<L, R> Mul<R> for Shape<L> {
    type Output = Intersectioned<Self, R>;

    fn mul(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}
