use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::functor::Fmap,
};

use crate::{Alias, Combine, Domains, Modify, Run};

impl_adt! {
    impl<A, F> Fmap<F> for Run<A> | Modify<A> | Domains<A> | Alias<A>
    where
        F: Clone + Closure<A>,
    {
        type Fmap = This<OutputT<F, A>>;

        fn fmap(self, f: F) -> Self::Fmap {
            This(f.clone().call(self.0))
        }
    }
}

impl<A, B, F, FM> Fmap<FM> for Combine<A, B, F>
where
    FM: Clone + Closure<A> + Closure<B> + Closure<F>,
{
    type Fmap = Combine<OutputT<FM, A>, OutputT<FM, B>, OutputT<FM, F>>;

    fn fmap(self, f: FM) -> Self::Fmap {
        Combine(
            f.clone().call(self.0),
            f.clone().call(self.1),
            f.call(self.2),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::glam::Vec2;
    use t_funk::{closure::Const, typeclass::functor::Fmap};

    use crate::{IsosurfaceS, Point, TranslateS};

    #[test]
    fn test_adt_fmap() {
        let adt = (TranslateS(Vec2::new(0.0, 0.0)), Point, IsosurfaceS(0.0));
        let mapped = adt.fmap(Const(()));
        assert_eq!(mapped, ((), (), ()));
    }
}

