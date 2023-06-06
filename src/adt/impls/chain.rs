use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{Combine, Modify, Sequence, Shape};

impl_adt! {
    impl<F, A, B, C> Chain<F> for Shape<A> | Modify<A> | Sequence<A, B> | Combine<A, B, C>
    where
        Self: Fmap<F>,
        FmapT<Self, F>: Mconcat,
    {
        type Chain = MconcatT<FmapT<Self, F>>;

        fn chain(self, f: F) -> Self::Chain {
            self.fmap(f).mconcat()
        }
    }
}

#[cfg(test)]
mod test {
    use t_funk::{
        closure::Const,
        collection::hlist::{Cons, Nil},
        macros::lift,
        typeclass::monad::Chain,
    };

    use crate::{shape, Done, Isosurface, Point};

    #[lift]
    fn make_list<A>(a: A) -> Cons<A, Nil> {
        Cons(a, Nil)
    }

    #[test]
    fn test_adt_monad() {
        let shape = shape() << Point << Isosurface(Const(0.2)) >> Done;
        //let list = shape.chain(MakeList);
    }
}
