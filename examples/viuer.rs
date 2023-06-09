use elysian::{
    adt, intersection, make_viuer, subtraction, union, Circle, DistGrad, Distance, Done, Evaluate,
    Gradient, Invert, Isosurface, Manifold, PosDistGrad, Saturate, Scale, Translate,
};
use glam::Vec2;
use image::{Luma, Pixel, Rgb};
use t_funk::{
    closure::{Closure, Compose},
    collection::set::Get,
    function::{FormatDebugMultiline, PrintLn, ResultUnwrap, Snd},
    macros::lift,
    op_chain::tap,
    typeclass::{arrow::Fanout, copointed::Copointed, functor::Fmap},
};

#[lift]
pub fn dist_to_luma<C>(c: C) -> Luma<f32>
where
    C: Get<Distance<f32>>,
{
    *Pixel::from_slice(&[c.get().fmap(Saturate).fmap(Invert).copoint()])
}

#[lift]
pub fn dist_grad_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<(Distance<f32>, Gradient<Vec2>)>,
{
    let (Distance(dist), Gradient(g)) = c.get();

    let c = if dist <= 0.0 {
        [g.x * 0.5 + 0.5, g.y * 0.5 + 0.5, 1.0 - dist]
    } else {
        [g.x * 0.5 + 0.5, g.y * 0.5 + 0.5, 0.0]
    };

    *Pixel::from_slice(&c)
}

#[lift]
pub fn viuer<S>(s: S)
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
        /*
        .fanout(
            make_viuer::<Dist, PosDistGrad, DistToLuma>(48, 48)
                .compose_l(ResultUnwrap),
        )
        */
        .fanout(
            make_viuer::<DistGrad<f32, Vec2>, PosDistGrad<Vec2, f32, Vec2>, DistGradToRgb>(48, 48)
                .compose_l(ResultUnwrap),
        )
        .compose_l(Snd)
        .call((PosDistGrad::default(), s));
}

fn main() {
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
        << Manifold << Isosurface(0.2_f32) 
        >> tap(Viuer)
        >> Done;
}
