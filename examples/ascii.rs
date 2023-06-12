use std::marker::PhantomData;

use elysian::{
    adt, union, AdtEnd, Circle, Context, ContextRasterString, Dist, Distance, Done, Evaluate,
    Isosurface, Manifold, Modify, PosDist, Position, Print, Raster, RasterToAscii, Rasterizer, Run,
    Scale, Then, Translate, ASCII_RAMP,
};
use glam::Vec2;
use t_funk::{macros::lift, op_chain::tap};

fn main() {
    pub type ShapeContextFrom = PosDist<Position<Vec2>, ()>;
    pub type ShapeContextTo = PosDist<(), Distance<f32>>;
    pub type RasterCtx =
        ContextRasterString<Context<ShapeContextFrom>, Raster<ShapeContextFrom>, String>;
    pub type Domain = Dist<f32>;

    #[lift]
    fn ascii<T>(t: T)
    where
        Then<
            Run<Modify<Rasterizer<T, ShapeContextFrom>>>,
            Then<Run<Modify<RasterToAscii<11, ShapeContextTo>>>, Then<Run<Modify<Print>>, AdtEnd>>,
        >: Evaluate<Domain, RasterCtx>,
    {
        let comp = adt()
            << Rasterizer::<T, ShapeContextFrom> {
                width: 48,
                height: 24,
                shape: t,
                context: Default::default(),
            }
            << RasterToAscii(ASCII_RAMP, PhantomData::<ShapeContextTo>)
            << Print
            >> Done;

        comp.evaluate(Default::default());
    }

    let shape_a = adt() << Translate(Vec2::new(0.8, -0.8)) << Circle(0.2_f32) >> tap(Ascii) >> Done;
    let shape_b = adt() << Translate(Vec2::new(0.8, 0.8)) << Circle(0.1_f32) >> tap(Ascii) >> Done;
    let shape_c = adt() << Translate(Vec2::new(0.0, 0.8)) << Circle(0.3_f32) >> tap(Ascii) >> Done;
    let shape_d =
        adt() << Translate(Vec2::new(0.0, -0.8)) << Circle(0.15_f32) >> tap(Ascii) >> Done;

    let combined = union() << shape_a << shape_b << shape_c << shape_d >> tap(Ascii) >> Done;

    let _shape = adt()
        << Translate(Vec2::new(0.25, 0.25))
        << Scale(0.5_f32)
        << combined
        << Isosurface(0.2_f32)
        << Manifold
        >> tap(Ascii)
        >> Done;
}
