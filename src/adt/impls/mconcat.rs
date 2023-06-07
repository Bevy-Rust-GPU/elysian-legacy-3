use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

use crate::{Combine, Nil, Sequence, Unit};

impl Mconcat for Nil {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

impl_adt! {
    impl<A, B, C> Mconcat for Unit<A> | Sequence<A, B>
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
    use t_funk::{
        closure::Const,
        r#do::Done,
        typeclass::{
            foldable::Foldl,
            functor::Fmap,
            monoid::Mconcat,
            monoid::Mempty,
            semigroup::{MappendF, Sum},
        },
    };

    use crate::{shape, Isosurface, Nil, Point, Sequence, Translate};

    #[test]
    fn test_adt_mconcat() {
        let adt = shape() << Translate(Const(0.0), Const(0.0)) << Point << Isosurface(0.0) >> Done;
        let foo = adt.fmap(Const(t_funk::collection::hlist::Cons(
            Sum(1),
            t_funk::collection::hlist::Nil,
        )));
        let baz = foo.foldl(MappendF, t_funk::collection::hlist::Nil::mempty());
        let bar = foo.mconcat();
    }
}
