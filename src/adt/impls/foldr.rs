use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl<F, Z> Foldr<F, Z> for End {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}

impl_adt! {
    impl<A, F, Z> Foldr<F, Z> for Input<A> | Field<A> | Output<A> | Modify<A>
    where
        F: Closure<(A, Z)>
    {
        type Foldr = OutputT<F, (A, Z)>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            f.call((self.0, z))
        }
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

    use crate::{Isosurface, Point, Translate, adt};

    #[lift]
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn test_adt_foldr() {
        let adt = adt() << Translate(Vec2::new(0.0, 0.0)) << Point << Isosurface(0.0) >> Done;
        let folded = adt.fmap(FormatDebug).foldr(Concat, String::default());

        assert_eq!(
            folded,
            "Isosurface(0.0)PointTranslate(Vec2(0.0, 0.0))"
        )
    }
}
