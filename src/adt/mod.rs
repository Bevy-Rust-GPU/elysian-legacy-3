//! Elysian ADT
//! Elysian = Input a
//! | Field b
//! | Output c
//! | Modify d
//! | Sequence [In|Field|Out|Modify]
//! | Combine Field|Shape|Combine Field|Shape|Combine f
//! where
//!   a: InputModifer
//!   b: FieldMorphism
//!   c: OutputModifier
//!   f: CombineFunction
//!
//! Example:
//!
//! Shape [
//!   In Translate -0.1 -0.3,
//!   Combine (
//!     Shape [
//!       In Translate 0.2 0.2,
//!       Field Point,
//!       Out Isosurface 0.3,
//!     ],
//!     Shape [
//!       In Translate -0.2 -0.2,
//!       Field Point,
//!       Out Isosurface 0.5,
//!       Out Manifold,
//!     ],
//!     Boolean(Lt),
//!   ),
//!   Out Isosurface 0.2,
//! ]
//!

mod algebra;
mod impls;
mod shape;
mod modify;

pub use algebra::*;
pub use impls::*;
pub use modify::*;
pub use shape::*;

mod bounds;
pub(crate) use bounds::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Unit<T>(pub T)
             | Sequence<A, B>(pub A, pub B)
             | Combine<A, B, F>(pub A, pub B, pub F);
);

pub use t_funk::r#do::Done;

#[cfg(test)]
mod test {
    use t_funk::{
        closure::{Closure, Compose, Const, Curry2},
        function::PrintLn,
    };

    use crate::{
        adt, intersection, shape, union, Ascii, Circle, Dist, Distance, Done, Evaluate, Get,
        LiftCombine, LiftEvaluate, LiftParam, PosDist, Rasterize, Translate, ASCII_RAMP, modify,
    };

    #[test]
    fn test_adt() {
        let shape_a = shape() << Translate(Const(-0.8), Const(-0.8)) << Circle(Const(0.2)) >> Done;
        let shape_b = shape() << Translate(Const(0.8), Const(0.8)) << Circle(Const(0.1)) >> Done;
        let shape_c = shape() << Translate(Const(0.0), Const(0.8)) << Circle(Const(0.3)) >> Done;
        let shape_d = shape() << Translate(Const(0.0), Const(-0.8)) << Circle(Const(0.15)) >> Done;

        let combined =
            union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> Done;

        let foo = adt() << combined >> modify() << Get::<Distance<f32>>::default() >> Done;

        let _foo = Evaluate::<Dist<f32>, PosDist<f32>>::evaluate(foo, PosDist::<f32>::default());

        panic!("{_foo:#?}");

        let _foo = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(
            combined
                .lift_param(PosDist::<f32>::default())
                .lift_combine(),
        )
        .call(PosDist::<f32>::default());

        let _foo = Rasterize::<(Distance<f32>, ()), PosDist<f32>>::default().call(combined);

        Rasterize::<(Distance<f32>, ()), PosDist<f32>> {
            width: 32,
            height: 32,
            ..Default::default()
        }
        .compose_l(Ascii.prefix2(ASCII_RAMP))
        .compose_l(PrintLn)
        .call(combined);
    }
}
