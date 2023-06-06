use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, B, F> Chain<F> for Input<A, B> | Field<A, B> | Output<A, B>
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

impl<F> Chain<F> for Nil {
    type Chain = Self;

    fn chain(self, _f: F) -> Self::Chain {
        self
    }
}

#[cfg(test)]
mod test {
    use t_funk::{
        closure::{Compose, Const},
        typeclass::{monad::Chain, copointed::CopointF}, collection::hlist::Cons,
    };

    use crate::{Field, Input, Isosurface, LiftAdtF, LiftShapeF, Nil, Output, Point, Translate};

    #[t_funk::macros::lift]
    fn make_list<A>(a: A) -> t_funk::collection::hlist::Cons<A, t_funk::collection::hlist::Nil> {
        t_funk::collection::hlist::Cons(a, t_funk::collection::hlist::Nil)
    }

    #[test]
    fn test_shape_monad() {
        let in_shape = Input(
            Translate(Const(0.5), Const(0.5)),
            Field(Point, Output(Isosurface(Const(0.5)), Nil)),
        );
        let in_list = Cons(Translate(Const(0.5), Const(0.5)), Cons(Point, Cons(Isosurface(Const(0.5)), t_funk::collection::hlist::Nil)));

        let out_list = in_shape.chain(MakeList);
        let out_shape = in_list.chain(LiftShapeF);

        assert_eq!(in_shape, out_shape);
        assert_eq!(in_list, out_list);
    }
}
