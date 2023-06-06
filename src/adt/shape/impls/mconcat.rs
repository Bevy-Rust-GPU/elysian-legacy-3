use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

use crate::{Field, Input, Output};

impl_adt! {
    impl<A, B> Mconcat for Input<A, B> | Field<A, B> | Output<A, B>
    where
        Self: Mempty + Foldl<MappendF, <Self as Mempty>::Mempty>,
    {
        type Mconcat = FoldlT<Self, MappendF, MemptyT<Self>>;

        fn mconcat(self) -> Self::Mconcat {
            self.foldl(MappendF::default(), Self::mempty())
        }
    }
}
