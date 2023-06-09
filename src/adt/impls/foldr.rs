use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl<F, Z> Foldr<F, Z> for AdtEnd {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}

impl<A, F, Z> Foldr<F, Z> for Run<A>
where
    F: Closure<(A, Z)>,
{
    type Foldr = OutputT<F, (A, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldr<F, Z> for Then<A, B> | Combine<A, B, C>
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
    use glam::Vec2;
    use t_funk::{
        function::FormatDebug,
        macros::lift,
        op_chain::Done,
        typeclass::{foldable::Foldr, functor::Fmap},
    };

    use crate::{adt, Isosurface, Point, Translate};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldr() {
        let adt =
            adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> adt() >> Done;
        let folded = adt.fmap(FormatDebug).foldr(Concat, String::default());

        assert_eq!(
            folded,
            "Input(Translate(Vec2(0.0, 0.0)), Field(Point, Output(Isosurface(0.0), ShapeEnd)))"
        )
    }
}
