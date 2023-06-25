use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{Alias, Combine, Modify, Run, Domains};

impl_adt! {
    impl<F, A, B, C> Chain<F> for Run<A> | Modify<A> | Domains<A> | Alias<A> | Combine<A, B, C>
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
    use glam::Vec2;
    use t_funk::{macros::lift, typeclass::functor::Fmap};

    use crate::{Distance, Get, Isosurface, LiftAdtF, Modify, Point, Run, Translate, Domains};

    #[lift]
    fn make_tuple<A>(a: A) -> (A,) {
        (a,)
    }

    #[test]
    fn test_adt_monad() {
        let from_shape = (
            Translate(Vec2::new(0.5, 0.5)),
            Point,
            Isosurface(0.2),
            Get::<Distance<f32>>::default(),
        );

        let to_shape = from_shape.fmap(LiftAdtF);

        assert_eq!(
            to_shape,
            (
                Modify(Translate(Vec2::new(0.5, 0.5))),
                Domains(Point),
                Modify(Isosurface(0.2)),
                Run(Get::<Distance<f32>>::default()),
            )
        );
    }
}
