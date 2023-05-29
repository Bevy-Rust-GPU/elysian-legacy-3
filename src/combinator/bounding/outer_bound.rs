use std::{marker::PhantomData, ops::BitXor};

use type_fields::t_funk::Gt;

use crate::{Boolean, Bounding, Shape};

impl<L, R> BitXor<R> for Shape<L> {
    type Output = Bounding<Self, R, Gt>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}

impl<L1, L2, O, R> BitXor<R> for Boolean<L1, L2, O> {
    type Output = Bounding<Self, R, Gt>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}

impl<L1, L2, O, R> BitXor<R> for Bounding<L1, L2, O> {
    type Output = Bounding<Self, R, Gt>;

    fn bitxor(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}

