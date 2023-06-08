use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl<F> Fmap<F> for End {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T, N, F> Fmap<F> for Then<T, N>
where
    T: Fmap<F>,
    N: Fmap<F>,
    F: Clone,
{
    type Fmap = Then<FmapT<T, F>, FmapT<N, F>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Then(self.0.fmap(f.clone()), self.1.fmap(f))
    }
}

impl<A, B, F, FM> Fmap<FM> for Combine<A, B, F>
where
    A: Fmap<FM>,
    B: Fmap<FM>,
    FM: Clone,
{
    type Fmap = Combine<FmapT<A, FM>, FmapT<B, FM>, F>;

    fn fmap(self, f: FM) -> Self::Fmap {
        Combine(self.0.fmap(f.clone()), self.1.fmap(f), self.2)
    }
}

impl_adt! {
    impl<A, F> Fmap<F> for Input<A> | Field<A> | Output<A> | Modify<A>
    where
        F: Clone + Closure<A>,
    {
        type Fmap = This<OutputT<F, A>>;

        fn fmap(self, f: F) -> Self::Fmap {
            This(f.clone().call(self.0))
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{closure::Const, op_chain::Done, typeclass::functor::Fmap};

    use crate::{adt, Field, Input, Isosurface, End, Output, Point, Then, Translate};

    #[test]
    fn test_adt_fmap() {
        let adt = adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> Done;
        let mapped = adt.fmap(Const(()));
        assert_eq!(
            mapped,
            Then(Input(()), Then(Field(()), Then(Output(()), End)))
        );
    }
}
