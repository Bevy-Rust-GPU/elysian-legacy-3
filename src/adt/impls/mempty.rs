use t_funk::{
    macros::impl_adt,
    typeclass::monoid::{Mempty, MemptyT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl_adt! {
    impl<A, B> Mempty for Run<A> | Then<A, B> | AdtEnd
    {
        type Mempty = AdtEnd;

        fn mempty() -> Self::Mempty {
            AdtEnd
        }
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
