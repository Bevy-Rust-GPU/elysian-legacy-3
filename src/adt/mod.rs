mod expand_alias;
mod impls;
mod into_monad;
mod into_tuple;
mod lift_adt;

pub use expand_alias::*;
pub use impls::*;
pub use into_monad::*;
pub use into_tuple::*;
pub use lift_adt::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ADT
      // Run a computation
      = Run<A>(pub A)
      // Lift a symbol to a context I/O function and run it
      | Modify<A>(pub A)
      // Lift a symbol to a set of fan-joined context I/O functions and run them
      | Domains<A>(pub A)
      // Expand to some other ADT structure
      | Alias<A>(pub A)
      // Combine two computations
      | Combine<A, B, F>(pub A, pub B, pub F);
);

#[cfg(test)]
mod test {

    use core::marker::PhantomData;

    use crate::{
        glam::{Vec2, Vec4},
        ImageWriter, DistGradToRgba,
    };
    use image::{ImageBuffer, Rgb, Rgba};
    use t_funk::{
        closure::{Closure, ComposeLF, Curry2, Div},
        function::{range, Id},
        macros::lift,
        typeclass::{foldable::Foldr, functor::Fmap, monad::Chain, semigroup::Mappend},
    };

    use crate::{
        circle, polynomial_smooth_overlay, ring, smooth_overlay, triangle, union,
        BlendPropertyDist, Color, ColorToRgba, Context, ContextRasterImage, DistGrad,
        DistGradToRgb, DistToRgba, Distance, Evaluate, EvaluateImpl, ExpandAlias, ExpandAliasF,
        FoldCombine, Gradient, Infinity, IntoTuple, Isomanifold, LiftAdt, LiftAdtF, LiftEvaluateF,
        LiftParamF, MakeProxy, MakeReplace, PolynomialSmoothOverlay, PosDistGradColor, Position,
        Raster, Rasterize, Ring, Scale, SetColor, ToImage, Translate, Viuer, BLACK, CYAN,
        TRANSPARENT, YELLOW,
    };

    pub type ShapeCtxFrom = PosDistGradColor<Position<Vec2>, (), (), Color<Vec4>>;
    pub type ShapeCtxTo = PosDistGradColor<(), Distance<f32>, Gradient<Vec2>, Color<Vec4>>;

    pub type RasterCtx = ContextRasterImage<
        Context<ShapeCtxFrom>,
        Raster<ShapeCtxFrom>,
        ImageBuffer<Rgba<f32>, Vec<f32>>,
    >;

    #[test]
    fn test_adt() {
        let shape_a = ring()
            .radius(0.8_f32)
            .width(0.1_f32)
            .translate(Vec2::splat(-0.2));

        let shape_b = ring()
            .radius(0.8_f32)
            .width(0.2_f32)
            .translate(Vec2::splat(0.2));

        let _shape_c = ring()
            .radius(0.8_f32)
            .width(0.3_f32)
            .translate(Vec2::X * 0.4);

        let _shape_d = ring()
            .radius(0.8_f32)
            .width(0.4_f32)
            .translate(Vec2::Y * -0.4);

        let combined = shape_a.replace::<Gradient<Vec2>>(shape_b);

        let shape = combined.into_tuple();
        let context = ShapeCtxFrom::default();

        let bar = shape.fmap(LiftAdtF);
        let bar = bar.fmap(LiftParamF.suffix2(context.clone()));
        let bar = bar.chain(ExpandAliasF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.fmap(LiftEvaluateF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.foldr(ComposeLF, Id);
        let _bar = bar.call(context);

        let _foo = EvaluateImpl::<DistGrad<f32, Vec2>, ShapeCtxFrom>::evaluate_impl(
            shape,
            Default::default(),
        );

        let _combined = shape_a.proxy::<Gradient<f32>>(shape_b);

        /*
        let input = PosDistColor::<(), (), Color<Vec3>>::default();
        let foo = positioned.lift_param(input.clone());
        let foo = LiftCombine::<Dist<f32>>::lift_combine(foo);
        let foo = LiftEvaluate::<Dist<f32>>::lift_evaluate(foo);
        let _foo = foo.call(input);
        let _foo =
            Evaluate::<Dist<f32>, PosDistColor<(), (), Color<Vec3>>>::evaluate(positioned, input);
        */

        shape
            .rasterize::<ShapeCtxFrom>(48, 48)
            .to_image::<ShapeCtxTo>(DistGradToRgb)
            .viuer::<ImageBuffer<Rgb<f32>, Vec<f32>>>()
            .evaluate::<DistGrad<f32, Vec2>>(RasterCtx::default());
    }

    #[test]
    fn test_composition() {
        let frag_size = 48.0_f32;
        let frag_recip = 1.0 / frag_size;

        let inner_line = 1.5_f32 / frag_size;
        let mid_line = 2.0_f32 / frag_size;
        let outer_line = 2.5_f32 / frag_size;

        let background = Infinity.color(TRANSPARENT);
        let tri = triangle().color(YELLOW);
        let circle = circle().color(CYAN);

        let lines = (
            range::<U1, U5, f32>()
                .fmap(Div(10.0_f32))
                .fmap(MakeRing.suffix2(inner_line))
                .fold_combine(union()),
            triangle().isomanifold(mid_line),
            ring().width(outer_line),
        )
            .fold_combine(union())
            .color(BLACK);

        let shape = (background, circle, tri, lines)
            //.fold_combine(overlay())
            .fold_combine((
                smooth_overlay().k(frag_recip),
                polynomial_smooth_overlay::<Gradient<Vec2>>(frag_recip),
                polynomial_smooth_overlay::<Color<Vec4>>(frag_recip),
            ))
            .scale(0.9_f32);

        shape
            .rasterize::<ShapeCtxFrom>(frag_size as usize, frag_size as usize)
            .to_image::<ShapeCtxTo>(ColorToRgba)
            .viuer::<ImageBuffer<Rgba<f32>, Vec<f32>>>()
            .evaluate::<DistGrad<f32, Vec2>>(RasterCtx::default());
    }

    #[lift]
    fn make_ring<T, U>(t: T, u: U) -> Ring<T, U> {
        Ring(t, u)
    }

    use t_funk::typenum::consts::{U1, U5};

    #[test]
    fn test_monadic_composition() {
        let frag_size = 48.0_f32;

        let inner_line = 1.5_f32 / frag_size;
        let mid_line = 2.0_f32 / frag_size;
        let outer_line = 2.5_f32 / frag_size;

        let shape = (
            range::<U1, U5, f32>()
                .fmap(Div(10.0_f32))
                .fmap(MakeRing.suffix2(inner_line))
                .fold_combine(union()),
            triangle().isomanifold(mid_line),
            ring().width(outer_line),
        )
            .fold_combine(union())
            .color(BLACK)
            .scale(0.9_f32);

        panic!("{shape:#?}");

        shape
            .rasterize::<ShapeCtxFrom>(frag_size as usize, frag_size as usize)
            .to_image::<ShapeCtxTo>(DistToRgba.prefix2(1.0))
            .viuer::<ImageBuffer<Rgba<f32>, Vec<f32>>>()
            .evaluate::<DistGrad<f32, Vec2>>(RasterCtx::default());
    }
}
