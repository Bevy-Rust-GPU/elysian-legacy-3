use elysian::{
    intersection, make_viuer, shape, subtraction, union, Circle, DistGrad, Distance, DistanceF32,
    Done, Evaluate, Gradient, GradientF32, Invert, Isosurface, Manifold, PosDistGrad, Saturate,
    Scale, Translate,
};
use image::{Luma, Pixel, Rgb};
use t_funk::{
    closure::{Closure, Compose, Const},
    collection::set::Get,
    function::{FormatDebugMultiline, PrintLn, ResultUnwrap, Snd},
    macros::lift,
    r#do::tap,
    typeclass::{arrow::Fanout, copointed::Copointed, functor::Fmap},
};

#[lift]
pub fn dist_to_luma<C>(c: C) -> Luma<f32>
where
    C: Get<DistanceF32>,
{
    *Pixel::from_slice(&[c.get().fmap(Saturate).fmap(Invert).copoint()])
}

#[lift]
pub fn dist_grad_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<(DistanceF32, GradientF32)>,
{
    let (Distance(dist), Gradient(gx, gy)) = c.get();

    let c = if dist <= 0.0 {
        [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 1.0 - dist]
    } else {
        [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 0.0]
    };

    *Pixel::from_slice(&c)
}

#[lift]
pub fn viuer<T>(t: T)
where
    T: core::fmt::Debug
        + Clone
        + Evaluate<DistGrad<f32>, PosDistGrad<f32>, Evaluate = PosDistGrad<f32>>,
{
    FormatDebugMultiline
        .compose_l(PrintLn)
        /*
        .fanout(
            make_viuer::<Dist, PosDistGrad, PosDistGrad, DistToLuma>(48, 48)
                .compose_l(ResultUnwrap),
        )
        */
        .fanout(
            make_viuer::<DistGrad<f32>, PosDistGrad<f32>, PosDistGrad<f32>, DistGradToRgb>(48, 48)
                .compose_l(ResultUnwrap),
        )
        .compose_l(Snd)
        .call(t);
}

fn main() {
    let shape_a =
        shape() << Translate(Const(-0.5), Const(-0.5)) << Circle(Const(1.2)) >> tap(Viuer) >> Done;

    let shape_b =
        shape() << Translate(Const(0.5), Const(0.5)) << Circle(Const(1.1)) >> tap(Viuer) >> Done;

    let shape_c =
        shape() << Translate(Const(0.0), Const(0.5)) << Circle(Const(1.3)) >> tap(Viuer) >> Done;

    let shape_d =
        shape() << Translate(Const(0.0), Const(-0.5)) << Circle(Const(1.15)) >> tap(Viuer) >> Done;

    let combined = intersection() << shape_a >> union() << shape_b << shape_c >> subtraction()
        << shape_d
        >> tap(Viuer)
        >> Done;

    let _shape = shape()
        << Translate(Const(0.25), Const(0.25))
        << Scale(Const(0.5))
        << combined
        << Manifold
        << Isosurface(Const(0.2))
        >> tap(Viuer)
        >> Done;
}
