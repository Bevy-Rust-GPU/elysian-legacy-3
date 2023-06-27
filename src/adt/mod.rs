mod expand_alias;
mod impls;
mod into_monad;
mod lift_adt;

pub use expand_alias::*;
pub use impls::*;
pub use into_monad::*;
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

pub use t_funk::op_chain::Done;

#[cfg(test)]
mod test {
    use glam::{Vec2, Vec3};
    use image::{ImageBuffer, Rgb};
    use t_funk::{
        closure::{Closure, Compose, ComposeLF, Curry2},
        function::{range, Div, Id, Mul},
        macros::lift,
        typeclass::{foldable::Foldr, functor::Fmap, monad::Chain},
    };

    use crate::{
        Circle, Color, ColorToRgb, Context, ContextRasterImage, Dist, DistColorToRgb, DistGrad,
        DistGradToRgb, Distance, Evaluate, ExpandAliasF, Gradient, Infinity, IntoMonad, Isosurface,
        LiftAdtF, LiftEvaluateF, LiftParamF, Manifold, NegInfinity, Overlay, PosDistGradColor,
        Position, Proxy, Raster, RasterToImage, Rasterizer, Replace, Ring, Set, Translate,
        Triangle, Union, UnionF, ViuerPrinter,
    };

    pub type ShapeCtxFrom = PosDistGradColor<Position<Vec2>, (), (), Color<Vec3>>;
    pub type ShapeCtxTo = PosDistGradColor<(), Distance<f32>, Gradient<Vec2>, Color<Vec3>>;

    pub type RasterCtx = ContextRasterImage<
        Context<ShapeCtxFrom>,
        Raster<ShapeCtxFrom>,
        ImageBuffer<Rgb<f32>, Vec<f32>>,
    >;

    #[test]
    fn test_adt() {
        let shape_a = (Translate(Vec2::new(-0.2, -0.2)), Ring(0.8_f32, 0.1_f32));
        let shape_b = (Translate(Vec2::new(0.2, 0.2)), Ring(0.8_f32, 0.2_f32));
        let _shape_c = (Translate(Vec2::new(0.0, 0.4)), Ring(0.8_f32, 0.3_f32));
        let _shape_d = (Translate(Vec2::new(0.0, -0.4)), Ring(0.8_f32, 0.4_f32));

        let combined = shape_a.replace::<Gradient<Vec2>>(shape_b);

        let shape = combined.into_monad();
        let context = ShapeCtxFrom::default();

        let bar = shape.fmap(LiftAdtF);
        let bar = bar.fmap(LiftParamF.suffix2(context.clone()));
        let bar = bar.chain(ExpandAliasF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.fmap(LiftEvaluateF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.foldr(ComposeLF, Id);
        let _bar = bar.call(context);

        let _foo =
            Evaluate::<DistGrad<f32, Vec2>, ShapeCtxFrom>::evaluate(shape, Default::default());

        let combined = shape_a.proxy::<Gradient<f32>>(shape_b);

        let _positioned = (Set(Position(Vec2::default())), combined);

        /*
        let input = PosDistColor::<(), (), Color<Vec3>>::default();
        let foo = positioned.lift_param(input.clone());
        let foo = LiftCombine::<Dist<f32>>::lift_combine(foo);
        let foo = LiftEvaluate::<Dist<f32>>::lift_evaluate(foo);
        let _foo = foo.call(input);
        let _foo =
            Evaluate::<Dist<f32>, PosDistColor<(), (), Color<Vec3>>>::evaluate(positioned, input);
        */
        let context = RasterCtx::default();

        let rasterizer = (
            Rasterizer::<_, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape,
                ..Default::default()
            },
            RasterToImage::<ShapeCtxTo, DistGradToRgb>::default(),
            ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default(),
            /*
            RasterToAscii(ASCII_RAMP, PhantomData::<PosDistGrad<Vec2, f32, Vec2>>),
            Print,
            */
        );

        Evaluate::<DistGrad<f32, Vec2>, RasterCtx>::evaluate(rasterizer, context);
    }

    #[test]
    fn test_composition() {
        let context = RasterCtx::default();

        let frag_size = 48.0_f32;

        let radius = 0.9_f32;

        let inner_line = 1.5_f32 / frag_size;
        let mid_line = 2.0_f32 / frag_size;
        let outer_line = 2.5_f32 / frag_size;

        let yellow = Vec3::new(1.0, 1.0, 0.0);
        let cyan = Vec3::new(0.0, 1.0, 1.0);
        let white = Vec3::ONE;
        let black = Vec3::ZERO;

        let neg_inf = (NegInfinity, Set(Color(white)));
        let triangle = (Triangle(radius), Set(Color(yellow)));
        let circle = (Circle(radius), Set(Color(cyan)));
        let lines = range::<U1, U5, f32>()
            .fmap(
                Div.suffix2(10.0_f32)
                    .compose_l(Mul.suffix2(radius))
                    .compose_l(MakeRing.suffix2(inner_line)),
            )
            .foldr(UnionF, Infinity)
            .union((
                Triangle(radius),
                Manifold,
                Isosurface(mid_line),
                Set(Color(black)),
            ))
            .union((Ring(radius, outer_line), Set(Color(black))));

        let shape = neg_inf.overlay(circle).overlay(triangle).overlay(lines);

        let shape = shape.into_monad();

        let rasterizer = (
            Rasterizer::<_, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape,
                ..Default::default()
            },
            RasterToImage(
                ColorToRgb,
                //DistGradToRgb,
                //DistToLuma,
                Default::default(),
            ),
            ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default(),
        );

        Evaluate::<DistGrad<f32, Vec2>, RasterCtx>::evaluate(rasterizer, context);
    }

    #[lift]
    fn make_ring<T, U>(t: T, u: U) -> Ring<T, U> {
        Ring(t, u)
    }

    use t_funk::typenum::consts::{U1, U5};

    #[test]
    fn test_monadic_composition() {
        let frag_size = 48.0_f32;

        let radius = 0.9_f32;

        let inner_line = 1.5_f32 / frag_size;
        let mid_line = 2.0_f32 / frag_size;
        let outer_line = 2.5_f32 / frag_size;

        let shape = range::<U1, U5, f32>()
            .fmap(
                Div.suffix2(10.0_f32)
                    .compose_l(Mul.suffix2(radius))
                    .compose_l(MakeRing.suffix2(inner_line)),
            )
            .foldr(UnionF, Infinity)
            .union((
                Triangle(radius),
                Manifold,
                Isosurface(mid_line),
                Set(Color(Vec3::ZERO)),
            ))
            .union((Ring(radius, outer_line), Set(Color(Vec3::ZERO))));

        let shape = shape.into_monad();

        let context = RasterCtx::default();

        let rasterizer = (
            Rasterizer::<_, ShapeCtxFrom> {
                width: frag_size as usize,
                height: frag_size as usize,
                shape,
                ..Default::default()
            },
            RasterToImage(
                //DistColorToRgb.prefix2((Vec3::ONE, frag_size)),
                DistColorToRgb.prefix2((Vec3::ONE, frag_size)),
                Default::default(),
            ),
            ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default(),
        );

        Evaluate::<DistGrad<f32, Vec2>, RasterCtx>::evaluate(rasterizer, context);
    }
}
