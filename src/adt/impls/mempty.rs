use t_funk::typeclass::monoid::{Mempty, MemptyT};

use crate::{Combine, Nil, Sequence, Unit};

impl Mempty for Nil {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}

impl<A, B> Mempty for Sequence<A, B> {
    type Mempty = Nil;

    fn mempty() -> Self::Mempty {
        Nil
    }
}

impl<A> Mempty for Unit<A>
where
    A: Mempty,
{
    type Mempty = MemptyT<A>;

    fn mempty() -> Self::Mempty {
        A::mempty()
    }
}

impl<A, B, C> Mempty for Combine<A, B, C>
where
    A: Mempty,
    B: Mempty<Mempty = MemptyT<A>>,
{
    type Mempty = MemptyT<A>;

    fn mempty() -> Self::Mempty {
        A::mempty()
    }
}
