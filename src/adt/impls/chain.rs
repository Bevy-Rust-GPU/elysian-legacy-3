use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{AdtEnd, Combine, Run, Then};

impl_adt! {
    impl<F, A, B, C> Chain<F> for AdtEnd | Run<A> | Then<A, B> | Combine<A, B, C>
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
    use t_funk::{
        collection::hlist::{Cons as HCons, Nil as HNil},
        macros::lift,
        typeclass::monad::Chain,
    };

    use crate::{
        adt, union, AdtEnd, Distance, Done, Get, Isosurface, LiftAdtF, Point, Run, Then, Translate,
    };

    #[lift]
    fn make_list<A>(a: A) -> HCons<A, HNil> {
        HCons(a, HNil)
    }

    #[test]
    fn test_adt_monad() {
        // Destructive transform from shape w/Combine to list
        let from_shape = adt() << Translate(Vec2::new(0.5, 0.5)) << Point << Isosurface(0.2)
            >> union()
            << (adt() << Point >> Done)
            << Get::<Distance<f32>>::default()
            >> Done;

        let to_list = from_shape.chain(MakeList);

        assert_eq!(
            to_list,
            HCons(
                Translate(Vec2::new(0.5, 0.5)),
                HCons(
                    Point,
                    HCons(
                        Isosurface(0.2),
                        HCons(Point, HCons(Get::<Distance<f32>>::default(), HNil)),
                    ),
                ),
            )
        );

        // Nondestructive transform from shape w/no Combine to list and back
        let from_shape = adt() << Translate(Vec2::new(0.5, 0.5)) << Point << Isosurface(0.2)
            >> adt()
            << Get::<Distance<f32>>::default()
            >> Done;

        let to_list = from_shape.chain(MakeList);

        assert_eq!(
            to_list,
            HCons(
                Translate(Vec2::new(0.5, 0.5)),
                HCons(
                    Point,
                    HCons(
                        Isosurface(0.2),
                        HCons(Get::<Distance<f32>>::default(), HNil),
                    ),
                ),
            )
        );

        let to_shape = to_list.chain(LiftAdtF);

        assert_eq!(
            to_shape,
            Then(
                Run(Translate(Vec2::new(0.5, 0.5))),
                Then(
                    Run(Point),
                    Then(
                        Run(Isosurface(0.2)),
                        Then(Run(Get::<Distance<f32>>::default()), AdtEnd),
                    ),
                ),
            )
        );

        assert_eq!(from_shape, to_shape);
    }
}
