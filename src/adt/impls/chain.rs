use t_funk::{
    macros::impl_adt,
    typeclass::{
        functor::{Fmap, FmapT},
        monad::Chain,
        monoid::{Mconcat, MconcatT},
    },
};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl_adt! {
    impl<F, A, B, C> Chain<F> for End | Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C>
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
        adt, union, Distance, Done, Field, Get, Input, Isosurface, LiftAdtF, Modify, End, Output,
        Point, Then, Translate,
    };

    #[lift]
    fn make_list<A>(a: A) -> HCons<A, HNil> {
        HCons(a, HNil)
    }

    #[test]
    fn test_adt_monad() {
        // Destructive transform from shape w/Combine to list
        let shape = adt() << Translate(Vec2::new(0.5, 0.5)) << Point << Isosurface(0.2) >> union()
            << (adt() << Point >> Done)
            >> adt()
            << Get::<Distance<f32>>::default()
            >> Done;

        let list = shape.chain(MakeList);

        assert_eq!(
            list,
            HCons(
                Translate(Vec2::new(0.5, 0.5)),
                HCons(
                    Point,
                    HCons(
                        Isosurface(0.2),
                        HCons(Point, HCons(Get::<Distance::<f32>>::default(), HNil,),),
                    ),
                ),
            )
        );

        // Nondestructive transform from shape w/no Combine to list and back
        let shape = adt()
            << Translate(Vec2::new(0.5, 0.5))
            << Point
            << Isosurface(0.2)
            << Get::<Distance<f32>>::default()
            >> Done;

        let list = shape.chain(MakeList);

        assert_eq!(
            list,
            HCons(
                Translate(Vec2::new(0.5, 0.5)),
                HCons(
                    Point,
                    HCons(
                        Isosurface(0.2),
                        HCons(Get::<Distance::<f32>>::default(), HNil,),
                    ),
                ),
            )
        );

        let shape = list.chain(LiftAdtF);

        assert_eq!(
            shape,
            Then(
                Input(Translate(Vec2::new(0.5, 0.5))),
                Then(
                    Field(Point),
                    Then(
                        Output(Isosurface(0.2)),
                        Then(Modify(Get::<Distance::<f32>>::default()), End),
                    ),
                ),
            )
        );
    }
}
