use elysian::{
    Circle, Color, Combine, Context, ContextRasterImage, Dist, DistColorToRgb, Distance, Evaluate,
    Isosurface, Manifold, PosDistColor, Position, Raster, RasterToImage, Rasterizer, Scale, Set,
    SubtractionS, Translate, UnionS, ViuerPrinter, IntoMonad, IntoMonadT
};
use glam::{Vec2, Vec3};
use image::{ImageBuffer, Rgb};
use t_funk::{closure::Closure, macros::lift, typeclass::monad::Identity};

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
        T: IntoMonad,
        (
            Rasterizer<IntoMonadT<T>, ShapeCtxFrom>,
            RasterToImage<ShapeCtxTo, DistColorToRgb>,
            ViuerPrinter<ImageBuffer<Rgb<f32>, Vec<f32>>>,
        ): Evaluate<Domains, RasterCtx>,
    {
        let t = t.into_monad();

        let comp = (
            Rasterizer::<IntoMonadT<T>, ShapeCtxFrom> {
                width: 48,
                height: 48,
                shape: t,
                context: Default::default(),
            },
            RasterToImage::<ShapeCtxTo, DistColorToRgb>::default(),
            ViuerPrinter::<ImageBuffer<Rgb<f32>, Vec<f32>>>::default(),
        );

        comp.evaluate(Default::default());
    }

    let shape_a = (
        Translate(Vec2::new(-0.5, -0.5)),
        Circle(1.2_f32),
        Set(Color(Vec3::X)),
    );
    Viuer.call(shape_a);

    let shape_b = (
        Translate(Vec2::new(0.5, 0.5)),
        Circle(1.1_f32),
        Set(Color(Vec3::Y)),
    );
    Viuer.call(shape_b);

    let shape_c = (
        Translate(Vec2::new(0.0, 0.5)),
        Circle(1.3_f32),
        Set(Color(Vec3::Z)),
    );
    Viuer.call(shape_c);

    let shape_d = (
        Translate(Vec2::new(0.0, -0.5)),
        Circle(1.15_f32),
        Set(Color(Vec3::ONE)),
    );
    Viuer.call(shape_d);

    let combined = Combine(
        Identity(Combine(
            Identity(Combine(shape_a, shape_b, Identity(UnionS))),
            shape_c,
            Identity(UnionS),
        )),
        shape_d,
        Identity(SubtractionS),
    );
    Viuer.call(Identity(combined));

    let shape = Scale(
        0.5_f32,
        (
            Translate(Vec2::new(0.25, 0.25)),
            combined,
            Manifold,
            Isosurface(0.5_f32),
        ),
    );
    Viuer.call(Identity(shape));
}
