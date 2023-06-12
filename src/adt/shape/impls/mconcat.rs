use crate::Shape;
use t_funk::typeclass::{
    foldable::{Foldl, FoldlT},
    monoid::{Mconcat, Mempty, MemptyT},
    semigroup::MappendF,
};

impl<A> Mconcat for Shape<A>
where
    A: Mempty,
    Self: Foldl<MappendF, MemptyT<A>>,
{
    type Mconcat = FoldlT<Self, MappendF, MemptyT<A>>;

    fn mconcat(self) -> Self::Mconcat {
        self.foldl(MappendF::default(), A::mempty())
    }
}
