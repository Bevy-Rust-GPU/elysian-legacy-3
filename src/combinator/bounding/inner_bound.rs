use std::{marker::PhantomData, ops::BitAnd};

use type_fields::t_funk::Lt;

use crate::{Boolean, Bounding, Shape};

impl<L, R> BitAnd<R> for Shape<L> {
    type Output = Bounding<Self, R, Lt>;

    fn bitand(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}

impl<L1, L2, O, R> BitAnd<R> for Boolean<L1, L2, O> {
    type Output = Bounding<Self, R, Lt>;

    fn bitand(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}

impl<L1, L2, O, R> BitAnd<R> for Bounding<L1, L2, O> {
    type Output = Bounding<Self, R, Lt>;

    fn bitand(self, rhs: R) -> Self::Output {
        Bounding(self, rhs, PhantomData)
    }
}
