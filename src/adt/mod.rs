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
    use glam::Vec2;
    use image::{ImageBuffer, Rgb};
    use t_funk::{
        closure::{Closure, ComposeLF, Curry2},
        function::Id,
        typeclass::{
            foldable::Foldr,
            functor::Fmap,
            monad::{Chain, Identity},
        },
    };

    use crate::{
        Circle, Combine, Context, ContextRasterImage, DistGrad, DistGradToRgb, Distance, Evaluate,
        ExpandAliasF, Gradient, IntoMonad, Isosurface, LiftAdtF, LiftEvaluateF, LiftParamF, Point,
        PosDistGrad, Position, Proxy, Raster, RasterToImage, Rasterizer, Replace, Set, Translate,
        UnionS, ViuerPrinter,
    };

    #[test]
    fn test_adt() {
        let shape_a = (Translate(Vec2::new(-0.2, -0.2)), Circle(0.8_f32));
        let shape_b = (Translate(Vec2::new(0.2, 0.2)), Circle(0.8_f32));
        let shape_c = (Translate(Vec2::new(0.0, 0.4)), Circle(0.8_f32));
        let shape_d = (Translate(Vec2::new(0.0, -0.4)), Circle(0.8_f32));

        let combined = shape_a.replace::<Gradient<Vec2>>(shape_b);

        let shape = combined.into_monad();
        let context = PosDistGrad::<Position<Vec2>, Distance<f32>, Gradient<Vec2>>::default();

        let bar = shape.fmap(LiftAdtF);
        let bar = bar.fmap(LiftParamF.suffix2(context.clone()));
        let bar = bar.chain(ExpandAliasF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.fmap(LiftEvaluateF::<DistGrad<f32, Vec2>>::default());
        let bar = bar.foldr(ComposeLF, Id);
        let bar = bar.call(context);

        let foo = Evaluate::<
            DistGrad<f32, Vec2>,
            PosDistGrad<Position<Vec2>, Distance<f32>, Gradient<Vec2>>,
        >::evaluate(shape, Default::default());

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

        pub type ShapeCtxFrom = PosDistGrad<Position<Vec2>, (), ()>;
        pub type ShapeCtxTo = PosDistGrad<(), Distance<f32>, Gradient<Vec2>>;

        //pub type RasterCtx = ContextRasterString<ShapeCtx, ShapeCtx>;
        pub type RasterCtx = ContextRasterImage<
            Context<ShapeCtxFrom>,
            Raster<ShapeCtxFrom>,
            ImageBuffer<Rgb<f32>, Vec<f32>>,
        >;

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
        let shape_a = (Translate(Vec2::new(-0.2, -0.2)), Point, Isosurface(0.8_f32));
        //let shape_a = (Translate(Vec2::new(0.8, -0.8)), Circle(0.2_f32));
        let shape_b = (Translate(Vec2::new(0.2, 0.2)), Point, Isosurface(0.8_f32));
        let combined = Combine(shape_a, shape_b, Identity(UnionS));

        let context = PosDistGrad::<Position<Vec2>, Distance<f32>, Gradient<Vec2>>::default();

        let shape = combined.into_monad();

        let foo = shape.fmap(LiftAdtF);
        let foo = foo.fmap(LiftParamF.suffix2(context));
        let foo = foo.chain(ExpandAliasF::<DistGrad<f32, Vec2>>::default());
        let foo = foo.fmap(LiftEvaluateF::<DistGrad<f32, Vec2>>::default());
        let foo = foo.foldr(ComposeLF, Id);
        let foo = foo.call(context);
        panic!("{foo:#?}");
    }
}
