use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Combine, Field, Input, Modify, Output, Sequence};

impl<T, F> Fmap<F> for Input<T>
where
    F: Closure<T>,
{
    type Fmap = Input<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Input(f.call(self.0))
    }
}

impl<T, F> Fmap<F> for Field<T>
where
    F: Closure<T>,
{
    type Fmap = Field<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Field(f.call(self.0))
    }
}

impl<T, F> Fmap<F> for Output<T>
where
    F: Closure<T>,
{
    type Fmap = Output<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Output(f.call(self.0))
    }
}

impl<T, F> Fmap<F> for Modify<T>
where
    F: Closure<T>,
{
    type Fmap = Modify<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Modify(f.call(self.0))
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
