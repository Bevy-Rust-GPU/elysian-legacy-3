use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

use crate::{Alias, Combine, Domains, Modify, Run};

impl_adt! {
    impl<A, B> Mconcat for Run<A> | Modify<A> | Domains<A> | Alias<A>
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

impl<A, B, C> Mconcat for Combine<A, B, C>
where
    A: Mempty,
    B: Mempty<Mempty = MemptyT<A>>,
    Self: Foldl<MappendF, MemptyT<A>>,
{
    type Mconcat = FoldlT<Self, MappendF, MemptyT<A>>;

    fn mconcat(self) -> Self::Mconcat {
        self.foldl(MappendF::default(), A::mempty())
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::Const,
        typeclass::{functor::Fmap, monoid::Mconcat, semigroup::Sum},
    };

    use crate::{Isosurface, Point, Translate};

    #[test]
    fn test_adt_mconcat() {
        let adt = (Translate(Vec2::new(0.0, 0.0)), Point, Isosurface(0.0));
        let foo = adt.fmap(Const((Sum(1),)));
        assert_eq!(foo.mconcat(), (Sum(1), Sum(1), Sum(1)));
    }
}
