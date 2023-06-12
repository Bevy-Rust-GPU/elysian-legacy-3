use elysian::{
    adt, intersection, subtraction, union, AdtEnd, Circle, Color, Context, ContextRasterImage,
    Dist, DistColorToRgb, Distance, Done, Evaluate, Isosurface, Manifold, Modify, PosDistColor,
    Position, Raster, RasterToImage, Rasterizer, Run, Scale, Set, Then, Translate, ViuerPrinter,
};
use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgb};
use t_funk::{macros::lift, op_chain::tap};

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
    fn viuer<T>(t: T)
    where
        Then<
            Run<Modify<Rasterizer<T, ShapeCtxFrom>>>,
            Then<
                Run<Modify<RasterToImage<ShapeCtxTo, DistColorToRgb>>>,
                Then<Run<Modify<ViuerPrinter<ImageBuffer<Rgb<f32>, Vec<f32>>>>>, AdtEnd>,
            >,
        >: Evaluate<Domains, RasterCtx>,
    {
        let comp = adt()
            << Rasterizer::<T, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape: t,
                context: Default::default(),
            }
            << RasterToImage::<ShapeCtxTo, DistColorToRgb>::default()
            << ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default()
            >> Done;

        comp.evaluate(Default::default());
    }

    let shape_a =
        adt() << Translate(Vec2::new(-0.5, -0.5)) << Circle(1.2_f32) << Set(Color(Vec3::X))
            >> tap(Viuer)
            >> Done;
    let shape_b = adt() << Translate(Vec2::new(0.5, 0.5)) << Circle(1.1_f32) << Set(Color(Vec3::Y))
        >> tap(Viuer)
        >> Done;
    let shape_c = adt() << Translate(Vec2::new(0.0, 0.5)) << Circle(1.3_f32) << Set(Color(Vec3::Z))
        >> tap(Viuer)
        >> Done;
    let shape_d =
        adt() << Translate(Vec2::new(0.0, -0.5)) << Circle(1.15_f32) << Set(Color(Vec3::ONE))
            >> tap(Viuer)
            >> Done;

    let combined = intersection() << shape_a >> union() << shape_b << shape_c >> subtraction()
        << shape_d
        >> tap(Viuer)
        >> Done;

    let _shape = adt()
        << Translate(Vec2::new(0.25, 0.25))
        << Scale(0.5_f32)
        << combined
        << Manifold
        << Isosurface(0.5_f32)
        >> tap(Viuer)
        >> Done;
}
