use t_funk::typeclass::monoid::Mempty;

use crate::{AdtEnd, Shape};

impl<A> Mempty for Shape<A> {
    type Mempty = AdtEnd;

    fn mempty() -> Self::Mempty {
        AdtEnd
    }
}
