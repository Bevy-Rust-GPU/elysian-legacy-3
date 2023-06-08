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

pub use algebra::*;
pub use impls::*;

mod bounds;
pub(crate) use bounds::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Input<A>(pub A)
             | Field<A>(pub A)
             | Output<A>(pub A)
             | Modify<A>(pub A)
             | Then<A, B>(pub A, pub B)
             | Combine<A, B, F>(pub A, pub B, pub F)
             | End;
);

pub use t_funk::op_chain::Done;

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::{Closure, Compose, Curry2},
        function::PrintLn,
    };

    use crate::{
        adt, intersection, union, Ascii, Circle, Dist, Distance, Done, Evaluate, Get, LiftCombine,
        LiftEvaluate, LiftParam, PosDist, Rasterize, Translate, ASCII_RAMP,
    };

    #[test]
    fn test_adt() {
        let shape_a = adt() << Translate(Vec2::new(-0.8, -0.8)) << Circle(0.2_f32) >> Done;
        let shape_b = adt() << Translate(Vec2::new(0.8, 0.8)) << Circle(0.1_f32) >> Done;
        let shape_c = adt() << Translate(Vec2::new(0.0, 0.8)) << Circle(0.3_f32) >> Done;
        let shape_d = adt() << Translate(Vec2::new(0.0, -0.8)) << Circle(0.15_f32) >> Done;

        let combined =
            union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> Done;

        let foo = adt() << combined << Get::<Distance<f32>>::default() >> Done;

        let _foo = Evaluate::<Dist<f32>, PosDist<Vec2, f32>>::evaluate(
            foo,
            PosDist::<Vec2, f32>::default(),
        );

        let _foo = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(
            combined
                .lift_param(PosDist::<Vec2, f32>::default())
                .lift_combine(),
        )
        .call(PosDist::<Vec2, f32>::default());

        let _foo = Rasterize::<(Distance<f32>, ())>::default().call((PosDist::default(), combined));

        Rasterize::<(Distance<f32>, ())> {
            width: 32,
            height: 32,
            ..Default::default()
        }
        .compose_l(Ascii.prefix2(ASCII_RAMP))
        .compose_l(PrintLn)
        .call((PosDist::default(), combined));
    }
}
