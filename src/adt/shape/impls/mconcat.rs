use crate::{Field, Input, Output, ShapeEnd};
use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

impl Mconcat for ShapeEnd {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

impl_adt! {
    impl<A, B> Mconcat for Input<A, B> | Field<A, B> | Output<A, B>
    where
        A: Mempty,
        Self: Foldl<MappendF, MemptyT<A>>,
    {
        type Mconcat = FoldlT<Self, MappendF, MemptyT<A>>;

        fn mconcat(self) -> Self::Mconcat {
            self.foldl(MappendF::default(), A::mempty())
        }
    }
}
