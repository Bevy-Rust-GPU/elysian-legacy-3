use t_funk::{
    macros::impl_adt,
    typeclass::monoid::{Mempty, MemptyT},
};

use crate::{Alias, Combine, Domains, Modify, Run};

impl_adt! {
    impl<A> Mempty for Run<A> | Modify<A> | Domains<A> | Alias<A>
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
