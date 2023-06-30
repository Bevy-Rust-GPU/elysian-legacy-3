use core::marker::PhantomData;

use elysian::glam::Vec2;
use elysian::{
    circle, Context, ContextRasterString, Dist, Distance, EvaluateImpl, FoldCombine, Isosurface,
    MakeOuterBound, Manifold, PosDist, Position, Print, Raster, RasterToAscii, Rasterizer, Scale,
    Translate, Union, ASCII_RAMP,
};
use t_funk::{macros::lift, typeclass::functor::Fmap};

fn main() {
    pub type ShapeContextFrom = PosDist<Position<Vec2>, ()>;
    pub type ShapeContextTo = PosDist<(), Distance<f32>>;
    pub type RasterCtx =
        ContextRasterString<Context<ShapeContextFrom>, Raster<ShapeContextFrom>, String>;
    pub type Domain = Dist<f32>;

    #[lift]
    fn ascii<T>(t: T) -> T
    where
        T: Clone,
        (
            Rasterizer<T, ShapeContextFrom>,
            RasterToAscii<11, ShapeContextTo>,
            Print,
        ): EvaluateImpl<Domain, RasterCtx>,
    {
        let comp = (
            Rasterizer::<T, ShapeContextFrom> {
                width: 48,
                height: 24,
                shape: t.clone(),
                context: Default::default(),
            },
            RasterToAscii(ASCII_RAMP, PhantomData::<ShapeContextTo>),
            Print,
        );

        comp.evaluate_impl(Default::default());

        t
    }

    let shape_a = circle().radius(0.2_f32).translate(Vec2::new(0.8, -0.8));
    let shape_b = circle().radius(0.1_f32).translate(Vec2::new(0.8, 0.8));
    let shape_c = circle().radius(0.3_f32).translate(Vec2::Y * 0.8);
    let shape_d = circle().radius(0.15_f32).translate(Vec2::Y * -0.8);

    let combined = (shape_a, shape_b, shape_c, shape_d)
        .fmap(ascii)
        .fold_combine(Union);
    ascii(combined);

    let combined = shape_a.outer_bound(combined);
    ascii(combined);

    let shape = combined
        .isosurface(0.2_f32)
        .manifold()
        .translate(Vec2::new(0.25, 0.25))
        .scale(0.5_f32);
    ascii(shape);
}
