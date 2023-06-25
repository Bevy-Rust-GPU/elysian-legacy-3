use std::marker::PhantomData;

use elysian::{
    Circle, Context, ContextRasterString, Dist, Distance, Evaluate, IntoMonad, IntoMonadT,
    Isosurface, Manifold, PosDist, Position, Print, Raster, RasterToAscii, Rasterizer,
    Scale, Translate, ASCII_RAMP, Union, OuterBound,
};
use glam::Vec2;
use t_funk::{closure::Closure, macros::lift};

fn main() {
    pub type ShapeContextFrom = PosDist<Position<Vec2>, ()>;
    pub type ShapeContextTo = PosDist<(), Distance<f32>>;
    pub type RasterCtx =
        ContextRasterString<Context<ShapeContextFrom>, Raster<ShapeContextFrom>, String>;
    pub type Domain = Dist<f32>;

    #[lift]
    fn ascii<T>(t: T)
    where
        T: IntoMonad,
        (
            Rasterizer<IntoMonadT<T>, ShapeContextFrom>,
            RasterToAscii<11, ShapeContextTo>,
            Print,
        ): Evaluate<Domain, RasterCtx>,
    {
        let t = t.into_monad();

        let comp = (
            Rasterizer::<IntoMonadT<T>, ShapeContextFrom> {
                width: 48,
                height: 24,
                shape: t,
                context: Default::default(),
            },
            RasterToAscii(ASCII_RAMP, PhantomData::<ShapeContextTo>),
            Print,
        );

        comp.evaluate(Default::default());
    }

    let shape_a = (Translate(Vec2::new(0.8, -0.8)), Circle(0.2_f32));
    Ascii.call(shape_a);

    let shape_b = (Translate(Vec2::new(0.8, 0.8)), Circle(0.1_f32));
    Ascii.call(shape_b);

    let shape_c = (Translate(Vec2::new(0.0, 0.8)), Circle(0.3_f32));
    Ascii.call(shape_c);

    let shape_d = (Translate(Vec2::new(0.0, -0.8)), Circle(0.15_f32));
    Ascii.call(shape_d);

    let combined = shape_a.union(shape_b).union(shape_c).union(shape_d);
    Ascii.call(combined);

    let combined = shape_a.outer_bound(combined);

    let shape = Scale(
        0.5_f32,
        (
            Translate(Vec2::new(0.25, 0.25)),
            combined,
            Isosurface(0.2_f32),
            Manifold,
        ),
    );

    Ascii.call(shape);
}
