use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl<F, Z> Foldl<F, Z> for AdtEnd {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}

impl<A, F, Z> Foldl<F, Z> for Run<A>
where
    F: Closure<(Z, A)>,
{
    type Foldl = OutputT<F, (Z, A)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldl<F, Z> for Then<A, B> | Combine<A, B, C>
    where
        F: Clone + Closure<(Z, A)>,
        B: Foldl<F, OutputT<F, (Z, A)>>,
    {
        type Foldl = FoldlT<B, F, OutputT<F, (Z, A)>>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            self.1.foldl(f.clone(), f.call((z, self.0)))
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        function::FormatDebug, macros::lift, op_chain::Done, typeclass::foldable::Foldl,
        typeclass::functor::Fmap,
    };

    use crate::{adt, Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldl() {
        let adt =
            adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> adt() >> Done;
        let mapped = adt.fmap(FormatDebug);
        let folded = mapped.foldl(Concat, String::default());

        assert_eq!(folded, "Run(Translate(Vec2(0.0, 0.0)))Run(Point)Run(Isosurface(0.0))")
    }
}
