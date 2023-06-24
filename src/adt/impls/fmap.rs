use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::Fmap, macros::impl_adt,
};

use crate::{Combine, Run, Alias};

impl_adt! {
    impl<A, F> Fmap<F> for Run<A> | Alias<A>
    where
        F: Clone + Closure<A>,
    {
        type Fmap = Run<OutputT<F, A>>;

        fn fmap(self, f: F) -> Self::Fmap {
            Run(f.clone().call(self.0))
        }
    }
}

impl<A, B, F, FM> Fmap<FM> for Combine<A, B, F>
where
    FM: Clone + Closure<A> + Closure<B>,
{
    type Fmap = Combine<OutputT<FM, A>, OutputT<FM, B>, F>;

    fn fmap(self, f: FM) -> Self::Fmap {
        Combine(f.clone().call(self.0), f.call(self.1), self.2)
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{closure::Const, typeclass::functor::Fmap};

    use crate::{Isosurface, Point, Translate};

    #[test]
    fn test_adt_fmap() {
        let adt = (Translate(Vec2::new(0.0, 0.0)), Point, Isosurface(0.0));
        let mapped = adt.fmap(Const(()));
        assert_eq!(mapped, ((), (), ()));
    }
}
