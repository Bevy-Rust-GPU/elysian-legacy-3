use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::Foldl,
};

use crate::{Alias, Domains, Modify, Run};

impl_adt! {
    impl<A, F, Z> Foldl<F, Z> for Run<A> | Modify<A> | Domains<A> | Alias<A>
    where
        F: Closure<(Z, A)>,
    {
        type Foldl = OutputT<F, (Z, A)>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            f.call((z, self.0))
        }
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

        assert_eq!(folded, "Translate(Vec2(0.0, 0.0))PointIsosurface(0.0)")
    }
}
