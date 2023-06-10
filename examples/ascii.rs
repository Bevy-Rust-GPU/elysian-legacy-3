use std::marker::PhantomData;

use elysian::{
    adt, intersection, union, AdtEnd, Circle, ContextRasterString, Done, Evaluate,
    Isosurface, Manifold, Modify, Print, RasterToAscii, Rasterizer, Run, Scale, Then,
    Translate, ASCII_RAMP, Dist, PosDist,
};
use glam::Vec2;
use t_funk::{macros::lift, op_chain::tap};

fn main() {
    pub type ShapeCtx = PosDist<Vec2, f32>;
    pub type RasterCtx = ContextRasterString<ShapeCtx, ShapeCtx>;
    pub type Domain = Dist<f32>;

    #[lift]
    fn ascii<T>(t: T)
    where
        Then<
            Run<Modify<Rasterizer<T, ShapeCtx>>>,
            Then<Run<Modify<RasterToAscii<11, ShapeCtx>>>, Then<Run<Modify<Print>>, AdtEnd>>,
        >: Evaluate<Domain, RasterCtx>,
    {
        let comp = adt()
            << Rasterizer::<T, ShapeCtx> {
                width: 48,
                height: 24,
                shape: t,
                context: Default::default(),
            }
            << RasterToAscii(ASCII_RAMP, PhantomData::<ShapeCtx>)
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
