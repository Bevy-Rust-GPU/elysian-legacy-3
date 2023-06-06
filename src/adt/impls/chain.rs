use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{Combine, Sequence, Unit};

impl_adt! {
    impl<F, A, B, C> Chain<F> for Unit<A> | Sequence<A, B> | Combine<A, B, C>
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
        closure::{Compose, Const},
        collection::hlist::{Cons, Nil},
        macros::lift,
        typeclass::monad::Chain,
    };

    use crate::{adt, modify, shape, Distance, Done, Get, Isosurface, Point, Translate};

    #[lift]
    fn make_list<A>(a: A) -> Cons<A, Nil> {
        Cons(a, Nil)
    }

    #[test]
    fn test_adt_monad() {
        let shape = shape() << Translate(Const(0.5), Const(0.5)) << Point << Isosurface(Const(0.2)) >> Done;
        let modifier = modify() << Get::<Distance<f32>>::default() >> Done;
        let foo = t_funk::typeclass::category::Compose::compose(shape, modifier);

        let list = shape.chain(MakeList.compose_l(MakeList));
        //let list = list.chain(LiftShapeF.compose_l(LiftAdtF));
    }
}
