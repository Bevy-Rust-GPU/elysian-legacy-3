use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{Combine, Nil, Sequence, Unit};

impl<F, Z> Foldl<F, Z> for Nil {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}

impl<T, F, Z> Foldl<F, Z> for Unit<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldl<F, Z> for Sequence<A, B> | Combine<A, B, C>
    where
        A: Foldl<F, Z>,
        B: Foldl<F, FoldlT<A, F, Z>>,
        F: Clone,
    {
        type Foldl = FoldlT<B, F, FoldlT<A, F, Z>>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            self.1.foldl(f.clone(), self.0.foldl(f, z))
        }
    }
}

#[cfg(test)]
mod test {
    use t_funk::{
        closure::Const,
        function::FormatDebug,
        macros::lift,
        r#do::Done,
        typeclass::{foldable::Foldl, functor::Fmap},
    };

    use crate::{shape, Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldl() {
        let adt = shape() << Translate(Const(0.0), Const(0.0)) << Point << Isosurface(0.0) >> Done;
        let folded = adt.fmap(FormatDebug).foldl(Concat, String::default());

        assert_eq!(
            folded,
            "Input(Translate(Const(0.0), Const(0.0)))Field(Point)Output(Isosurface(0.0))"
        )
    }
}
