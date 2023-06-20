use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl<F> Fmap<F> for AdtEnd {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<A, F> Fmap<F> for Run<A>
where
    F: Clone + Closure<A>,
{
    type Fmap = Run<OutputT<F, A>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Run(f.clone().call(self.0))
    }
}

impl<T, N, F> Fmap<F> for Then<T, N>
where
    F: Closure<T>,
    N: Fmap<F>,
    F: Clone,
{
    type Fmap = Then<OutputT<F, T>, FmapT<N, F>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Then(f.clone().call(self.0), self.1.fmap(f))
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
    use t_funk::{closure::Const, op_chain::Done, typeclass::functor::Fmap};

    use crate::{adt, AdtEnd, Isosurface, Point, Then, Translate};

    #[test]
    fn test_adt_fmap() {
        let adt =
            adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> adt() >> Done;
        let mapped = adt.fmap(Const(()));
        assert_eq!(mapped, Then((), Then((), Then((), AdtEnd))));
    }
}
