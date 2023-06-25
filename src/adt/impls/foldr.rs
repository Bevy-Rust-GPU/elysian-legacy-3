use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::Foldr,
};

use crate::{Alias, Domains, Modify, Run};

impl_adt! {
    impl<A, F, Z> Foldr<F, Z> for Run<A> | Modify<A> | Domains<A> | Alias<A>
    where
        F: Closure<(A, Z)>,
    {
        type Foldr = OutputT<F, (A, Z)>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            f.call((self.0, z))
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        function::FormatDebug,
        macros::lift,
        typeclass::{foldable::Foldr, functor::Fmap},
    };

    use crate::{Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldr() {
        let adt = (Translate(Vec2::new(0.0, 0.0)), Point, Isosurface(0.0));
        let folded = adt.fmap(FormatDebug).foldr(Concat, String::default());

        assert_eq!(folded, "Translate(Vec2(0.0, 0.0))PointIsosurface(0.0)")
    }
}
