use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::{Foldl, FoldlT}, macros::impl_adt,
};

use crate::{Combine, Run, Alias};

impl_adt! {
    impl<A, F, Z> Foldl<F, Z> for Run<A> | Alias<A>
    where
        F: Closure<(Z, A)>,
    {
        type Foldl = OutputT<F, (Z, A)>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            f.call((z, self.0))
        }
    }
}

impl<A, B, C, F, Z> Foldl<F, Z> for Combine<A, B, C>
where
    F: Clone + Closure<(Z, A)>,
    B: Foldl<F, OutputT<F, (Z, A)>>,
{
    type Foldl = FoldlT<B, F, OutputT<F, (Z, A)>>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        self.1.foldl(f.clone(), f.call((z, self.0)))
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        function::FormatDebug, macros::lift, typeclass::foldable::Foldl, typeclass::functor::Fmap,
    };

    use crate::{Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldl() {
        let adt = (Translate(Vec2::new(0.0, 0.0)), Point, Isosurface(0.0));
        let mapped = adt.fmap(FormatDebug);
        let folded = mapped.foldl(Concat, String::default());

        assert_eq!(
            folded,
            "Translate(Vec2(0.0, 0.0))PointIsosurface(0.0)"
        )
    }
}
