use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Combine, Nil, Sequence, Unit};

impl<F> Fmap<F> for Nil {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

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

#[cfg(test)]
mod test {
    use t_funk::{closure::Const, r#do::Done, typeclass::functor::Fmap};

    use crate::{shape, Isosurface, Nil, Point, Sequence, Translate, Unit};

    #[test]
    fn test_adt_fmap() {
        let adt = shape() << Translate(Const(0.0), Const(0.0)) << Point << Isosurface(0.0) >> Done;
        let mapped = adt.fmap(Const(()));
        assert_eq!(
            mapped,
            Sequence(Unit(()), Sequence(Unit(()), Sequence(Unit(()), Nil)))
        );
    }
}
