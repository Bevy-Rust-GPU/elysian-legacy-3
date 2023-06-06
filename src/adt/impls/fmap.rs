use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Combine, Sequence, Unit};

impl<T, F> Fmap<F> for Unit<T>
where
    F: Closure<T>,
{
    type Fmap = Unit<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Unit(f.call(self.0))
    }
}

impl<T, N, F> Fmap<F> for Sequence<T, N>
where
    T: Fmap<F>,
    N: Fmap<F>,
    F: Clone,
{
    type Fmap = Sequence<FmapT<T, F>, FmapT<N, F>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Sequence(self.0.fmap(f.clone()), self.1.fmap(f))
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
