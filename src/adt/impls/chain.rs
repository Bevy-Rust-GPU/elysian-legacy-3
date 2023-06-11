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
    use std::marker::PhantomData;

    use glam::Vec2;
    use t_funk::{
        collection::hlist::{Cons as HCons, Nil as HNil},
        macros::lift,
        typeclass::monad::Chain,
    };

    use crate::{
        adt, union, AdtEnd, Get, Distance, Done, Field, Input, Isosurface, LiftAdtF, Run,
        Output, Point, ShapeEnd, Then, Translate,
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
                Input(
                    Translate(Vec2::new(0.5, 0.5)),
                    Field(Point, Output(Isosurface(0.2), ShapeEnd),),
                ),
                HCons(
                    Field(Point, ShapeEnd),
                    HCons(Get(PhantomData::<Distance::<f32>>), HNil),
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
                Input(
                    Translate(Vec2::new(0.5, 0.5)),
                    Field(Point, Output(Isosurface(0.2), ShapeEnd),),
                ),
                HCons(Get(PhantomData::<Distance<f32>>), HNil),
            )
        );

        let to_shape = to_list.chain(LiftAdtF);

        assert_eq!(
            to_shape,
            Then(
                Run(Input(
                    Translate(Vec2::new(0.5, 0.5)),
                    Field(Point, Output(Isosurface(0.2), ShapeEnd),),
                ),),
                Then(Run(Get(PhantomData::<Distance::<f32>>)), AdtEnd,),
            )
        );

        assert_eq!(from_shape, to_shape);
    }
}
