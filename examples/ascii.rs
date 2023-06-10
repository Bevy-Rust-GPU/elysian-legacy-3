use std::marker::PhantomData;

use elysian::{
    adt, intersection, union, AdtEnd, Circle, ContextRasterString, DistGrad, Done, Evaluate,
    Isosurface, Manifold, Modify, PosDistGrad, Print, RasterToAscii, Rasterizer, Run, Scale, Then,
    Translate, ASCII_RAMP,
};
use glam::Vec2;
use t_funk::{macros::lift, op_chain::tap};

fn main() {
    pub type ShapeCtx = PosDistGrad<Vec2, f32, Vec2>;
    pub type RasterCtx = ContextRasterString<ShapeCtx, ShapeCtx>;

    #[lift]
    fn ascii<T>(t: T)
    where
        Then<
            Run<Modify<Rasterizer<T, PosDistGrad<Vec2, f32, Vec2>>>>,
            Then<
                Run<Modify<RasterToAscii<11, PosDistGrad<Vec2, f32, Vec2>>>>,
                Then<Run<Modify<Print>>, AdtEnd>,
            >,
        >: Evaluate<DistGrad<f32, Vec2>, RasterCtx>,
    {
        let comp = adt()
            << Rasterizer::<T, PosDistGrad<Vec2, f32, Vec2>> {
                width: 48,
                height: 24,
                shape: t,
                context: Default::default(),
            }
            << RasterToAscii(ASCII_RAMP, PhantomData::<PosDistGrad<Vec2, f32, Vec2>>)
            << Print
            >> Done;

        comp.evaluate(Default::default());
    }

    let shape_a = adt() << Translate(Vec2::new(0.8, -0.8)) << Circle(0.2_f32) >> tap(Ascii) >> Done;
    let shape_b = adt() << Translate(Vec2::new(0.8, 0.8)) << Circle(0.1_f32) >> tap(Ascii) >> Done;
    let shape_c = adt() << Translate(Vec2::new(0.0, 0.8)) << Circle(0.3_f32) >> tap(Ascii) >> Done;
    let shape_d =
        adt() << Translate(Vec2::new(0.0, -0.8)) << Circle(0.15_f32) >> tap(Ascii) >> Done;

    let combined =
        union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> tap(Ascii) >> Done;

    let _shape = adt()
        << Translate(Vec2::new(0.25, 0.25))
        << Scale(0.5_f32)
        << combined
        << Isosurface(0.2_f32)
        << Manifold
        >> tap(Ascii)
        >> Done;
}
