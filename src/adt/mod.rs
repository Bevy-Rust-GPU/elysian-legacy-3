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
mod modify;
mod shape;

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
    pub struct ADT
      // Run a computation
      = Run<A>(pub A)
      // Sequence two computations
      | Then<A, B>(pub A, pub B)
      // Combine two computations
      | Combine<A, B, F>(pub A, pub B, pub F)
      // Terminating type
      | AdtEnd;
);

pub use t_funk::op_chain::Done;

#[cfg(test)]
mod test {

    use glam::{Vec2, Vec3};
    use image::{ImageBuffer, Rgb, Luma};

    use t_funk::closure::{Closure, Const};

    use crate::{
        adt, intersection, union, Circle, Color, ContextRasterImage, Set, Dist, DistColor,
        DistColorToRgb, DistGrad, DistGradToRgb, Done, Evaluate, InvertGradient, LiftCombine,
        LiftEvaluate, LiftParam, PosDistColor, PosDistGrad, RasterToImage, Rasterizer, Translate,
        ViuerPrinter, DistToLuma,
    };

    #[test]
    fn test_adt() {
        let shape_a = adt()
            << Translate(Vec2::new(-0.8, -0.8))
            << Circle(0.2_f32)
            << Set(Const(Color(Vec3::X)))
            >> Done;
        let shape_b = adt()
            << Translate(Vec2::new(0.8, 0.8))
            << Circle(0.1_f32)
            << Set(Const(Color(Vec3::Y)))
            >> Done;
        let shape_c = adt()
            << Translate(Vec2::new(0.0, 0.8))
            << Circle(0.3_f32)
            << Set(Const(Color(Vec3::Z)))
            >> Done;
        let shape_d = adt()
            << Translate(Vec2::new(0.0, -0.8))
            << Circle(0.15_f32)
            << Set(Const(Color(Vec3::ONE)))
            >> Done;

        /*
        let combined =
            union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> Done;
        */

        let combined =
            union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> Done;

        let flipped = adt() << combined << InvertGradient >> Done;

        pub type ShapeCtx = PosDistColor<Vec2, f32, Vec3>;

        //pub type RasterCtx = ContextRasterString<ShapeCtx, ShapeCtx>;
        pub type RasterCtx = ContextRasterImage<ShapeCtx, ShapeCtx, Rgb<f32>, Vec<f32>>;

        let context = RasterCtx::default();

        let rasterizer = adt()
            << Rasterizer::<_, ShapeCtx> {
                width: 48,
                height: 48,
                shape: combined,
                ..Default::default()
            }
            << RasterToImage::<ShapeCtx, DistColorToRgb>::default()
            << ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default()
        /*
            << RasterToAscii(ASCII_RAMP, PhantomData::<PosDistGrad<Vec2, f32, Vec2>>)
            << Print
        */
            >> Done;

        let foo = combined.lift_param(context.clone());
        let foo = LiftCombine::<Dist<f32>>::lift_combine(foo);
        let foo = LiftEvaluate::<Dist<f32>>::lift_evaluate(foo);
        let foo = foo.call(ShapeCtx::default());

        Evaluate::<Dist<f32>, RasterCtx>::evaluate(rasterizer, context);
        //panic!("{foo:#?}");
        panic!();

        /*
        Evaluate::<Dist<f32>, ContextRaster<PosDist<Vec2, f32>, PosDist<Vec2, f32>>>::evaluate(
            rasterizer,
            context,
        );
        */
    }
}
