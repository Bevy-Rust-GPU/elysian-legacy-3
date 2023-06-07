use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{Combine, Nil, Sequence, Unit};

impl<F, Z> Foldr<F, Z> for Nil {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}

impl<T, F, Z> Foldr<F, Z> for Unit<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldr<F, Z> for Sequence<A, B> | Combine<A, B, C>
    where
        A: Foldr<F, Z>,
        B: Foldr<F, FoldrT<A, F, Z>>,
        F: Clone,
    {
        type Foldr = FoldrT<B, F, FoldrT<A, F, Z>>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            self.1.foldr(f.clone(), self.0.foldr(f, z))
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
        typeclass::{foldable::Foldr, functor::Fmap},
    };

    use crate::{shape, Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldr() {
        let adt = shape() << Translate(Const(0.0), Const(0.0)) << Point << Isosurface(0.0) >> Done;
        let folded = adt.fmap(FormatDebug).foldr(Concat, String::default());

        assert_eq!(
            folded,
            "Output(Isosurface(0.0))Field(Point)Input(Translate(Const(0.0), Const(0.0)))"
        )
    }
}
