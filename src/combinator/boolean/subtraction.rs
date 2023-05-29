use std::{marker::PhantomData, ops::Sub};

use type_fields::t_funk::{Composed, Firsted, Gt, Neg};

use crate::{Boolean, Shape};

pub type Subtract = Composed<Gt, Firsted<Neg>>;
pub type Subtractioned<T, U> = Boolean<T, U, Subtract>;

pub trait Subtraction<T>: Sized {
    fn subtract(self, t: T) -> Subtractioned<Self, T> {
        Boolean(self, t, PhantomData)
    }
}

impl<L, R> Subtraction<R> for L {}

impl<L1, L2, O, R> Sub<R> for Boolean<L1, L2, O> {
    type Output = Subtractioned<Self, R>;

    fn sub(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}

impl<L, R> Sub<R> for Shape<L> {
    type Output = Subtractioned<Self, R>;

    fn sub(self, rhs: R) -> Self::Output {
        Boolean(self, rhs, PhantomData)
    }
}
