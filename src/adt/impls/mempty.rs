use t_funk::{
    macros::impl_adt,
    typeclass::monoid::{Mempty, MemptyT},
};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl Mempty for End {
    type Mempty = End;

    fn mempty() -> Self::Mempty {
        End
    }
}

impl_adt! {
    impl<A, B> Mempty for Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B>
    where
        A: Mempty,
    {
        type Mempty = MemptyT<A>;

        fn mempty() -> Self::Mempty {
            A::mempty()
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
