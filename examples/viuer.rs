use elysian::{
    adt, intersection, subtraction, union, AdtEnd, Circle, ContextRasterImage,
    DistGrad, DistGradToRgb, Done, Evaluate, InvertGradient, Isosurface, Manifold, Modify,
    PosDistGrad, RasterToImage, Rasterizer, Run, Scale, Then, Translate, ViuerPrinter,
};
use glam::Vec2;
use image::{ImageBuffer, Rgb};
use t_funk::{
    closure::{Closure, Compose},
    function::{FormatDebugMultiline, PrintLn, ResultUnwrap, Snd},
    macros::lift,
    op_chain::tap,
    typeclass::arrow::Fanout,
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
    pub type ShapeCtx = PosDistGrad<Vec2, f32, Vec2>;
    pub type RasterCtx = ContextRasterImage<ShapeCtx, ShapeCtx, Rgb<f32>, Vec<f32>>;

    #[lift]
    fn viuer<T>(t: T)
    where
        Then<
            Run<Modify<Rasterizer<T, PosDistGrad<Vec2, f32, Vec2>>>>,
            Then<
                Run<Modify<RasterToImage<PosDistGrad<Vec2, f32, Vec2>, DistGradToRgb>>>,
                Then<Run<Modify<ViuerPrinter<ImageBuffer<Rgb<f32>, Vec<f32>>>>>, AdtEnd>,
            >,
        >: Evaluate<DistGrad<f32, Vec2>, RasterCtx>,
    {
        let comp = adt()
            << Rasterizer::<T, PosDistGrad<Vec2, f32, Vec2>> {
                width: 48,
                height: 48,
                shape: t,
                context: Default::default(),
            }
            << RasterToImage::<PosDistGrad<Vec2, f32, Vec2>, DistGradToRgb>::default()
            << ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default()
            >> Done;

        comp.evaluate(Default::default());
    }

    let shape_a =
        adt() << Translate(Vec2::new(-0.5, -0.5)) << Circle(1.2_f32) >> tap(Viuer) >> Done;
    let shape_b = adt() << Translate(Vec2::new(0.5, 0.5)) << Circle(1.1_f32) >> tap(Viuer) >> Done;
    let shape_c = adt() << Translate(Vec2::new(0.0, 0.5)) << Circle(1.3_f32) >> tap(Viuer) >> Done;
    let shape_d =
        adt() << Translate(Vec2::new(0.0, -0.5)) << Circle(1.15_f32) >> tap(Viuer) >> Done;

    let combined = intersection() << shape_a >> union() << shape_b << shape_c >> subtraction()
        << shape_d
        >> tap(Viuer)
        >> Done;

    let _shape = adt()
        << Translate(Vec2::new(0.25, 0.25))
        << Scale(0.5_f32)
        << combined
        << Manifold
        << Isosurface(0.2_f32)
        << InvertGradient
        >> tap(Viuer)
        >> Done;
}
