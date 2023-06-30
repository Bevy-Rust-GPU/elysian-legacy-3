use elysian::glam::{Vec2, Vec3};
use elysian::{
    Circle, Color, Context, ContextRasterImage, Dist, DistColorToRgb, Distance, EvaluateImpl,
    FoldCombine, Isomanifold, MakeSubtraction, PosDistColor, Position, Raster, RasterToImage,
    Rasterizer, Scale, Set, Translate, Union, ViuerPrinter,
};
use image::{ImageBuffer, Rgb};
use t_funk::{
    closure::{Curry2, Curry2A},
    macros::lift,
    typeclass::functor::Fmap,
};

// TODO: Reimplement printing behaviour
/*
pub fn viuer_old<S>(s: S)
where
    S: core::fmt::Debug
        + Clone
        + Evaluate<
            DistGrad<f32, Vec2>,
            PosDistGrad<Vec2, f32, Vec2>,
            Evaluate = PosDistGrad<Vec2, f32, Vec2>,
        >,
{
    FormatDebugMultiline
        .compose_l(PrintLn)
        .fanout(
            make_viuer::<DistGrad<f32, Vec2>, PosDistGrad<Vec2, f32, Vec2>, DistGradToRgb>(48, 48)
                .compose_l(ResultUnwrap),
        )
        .compose_l(Snd)
        .call((PosDistGrad::default(), s));
}
*/

fn main() {
    pub type Domains = Dist<f32>;
    pub type ShapeCtxFrom = PosDistColor<Position<Vec2>, (), Color<Vec3>>;
    pub type ShapeCtxTo = PosDistColor<(), Distance<f32>, Color<Vec3>>;
    pub type RasterCtx = ContextRasterImage<
        Context<ShapeCtxFrom>,
        Raster<ShapeCtxFrom>,
        ImageBuffer<Rgb<f32>, Vec<f32>>,
    >;

    #[lift]
    fn viuer<T>(t: T) -> T
    where
        T: Clone,
        (
            Rasterizer<T, ShapeCtxFrom>,
            RasterToImage<ShapeCtxTo, Curry2A<DistColorToRgb, (Vec3, f32)>>,
            ViuerPrinter<ImageBuffer<Rgb<f32>, Vec<f32>>>,
        ): EvaluateImpl<Domains, RasterCtx>,
    {
        let comp = (
            Rasterizer::<T, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape: t.clone(),
                context: Default::default(),
            },
            RasterToImage(
                DistColorToRgb.prefix2((Vec3::ZERO, 48.0)),
                Default::default(),
            ),
            ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default(),
        );

        comp.evaluate_impl(Default::default());

        t
    }

    let shape_a = Circle(1.2_f32)
        .translate(Vec2::new(-0.5, -0.5))
        .set(Color(Vec3::X));

    let shape_b = Circle(1.1_f32)
        .translate(Vec2::new(0.5, 0.5))
        .set(Color(Vec3::Y));

    let shape_c = Circle(1.3_f32)
        .translate(Vec2::new(0.0, 0.5))
        .set(Color(Vec3::Z));

    let shape_d = Circle(1.15_f32)
        .translate(Vec2::new(0.0, -0.5))
        .set(Color(Vec3::ONE));

    (shape_a, shape_b, shape_c, shape_d).fmap(viuer);

    let combined = (shape_a, shape_b, shape_c)
        .fold_combine(Union)
        .subtraction(shape_d);
    viuer(combined);

    let shape = combined
        .translate(Vec2::new(0.25, 0.25))
        .isomanifold(0.5_f32)
        .scale(0.5_f32);

    viuer(shape);
}
