use t_funk::{typeclass::{
    applicative::{Apply, ApplyT},
    functor::{Fmap, FmapT},
    semigroup::{Mappend, MappendT},
}, macros::impl_adt};

use crate::{Combine, Run, Alias, Modify, Domains};

impl_adt!{
    impl<A, T> Apply<T> for Run<A> | Modify<A> | Domains<A> | Alias<A>
    where
        T: Fmap<A>,
    {
        type Apply = FmapT<T, A>;

        fn apply(self, t: T) -> Self::Apply {
            t.fmap(self.0)
        }
    }
}

impl<A, B, F, T> Apply<T> for Combine<A, B, F>
where
    T: Clone + Fmap<A>,
    B: Apply<T>,
    FmapT<T, A>: Mappend<ApplyT<B, T>>,
{
    type Apply = MappendT<FmapT<T, A>, ApplyT<B, T>>;

    fn apply(self, t: T) -> Self::Apply {
        t.clone().fmap(self.0).mappend(self.1.apply(t))
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::{Const, Curry2},
        function::Mul,
        typeclass::{applicative::Apply, functor::Fmap},
    };

    use crate::{Distance, Get, Isosurface, Point, Translate};

    #[test]
    fn test_adt_apply() {
        let shape = (
            Translate(Vec2::new(0.5, 0.5)),
            Point,
            Isosurface(0.2_f32),
            Get::<Distance<f32>>::default(),
        );

        let funcs = shape.fmap(Const(Mul.suffix2(2)));
        let vals = funcs.apply((2, 3));
        assert_eq!(vals, ((4, 6), (4, 6), (4, 6), (4, 6)));
    }
}
