use elysian::{
    make_viuer, Circle, DistGrad, Distance, DistanceF32, Do, Done, Gradient, GradientF32, Invert,
    Isosurface, Manifold, PosDistGrad, Saturate, Scale, Translate,
};
use image::{Luma, Pixel, Rgb};
use t_funk::{
    closure::{Compose, Const},
    collection::set::Get,
    function::{DebugMultilineF, Function, PrintLn, ResultUnwrap, Snd},
    macros::Closure,
    typeclass::{arrow::Fanout, copointed::Copointed, functor::Fmap},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct DistToLuma;

impl<C> Function<C> for DistToLuma
where
    C: Get<DistanceF32>,
{
    type Output = Luma<f32>;

    fn call(input: C) -> Self::Output {
        *Pixel::from_slice(&[input.get().fmap(Saturate).fmap(Invert).copoint()])
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct DistGradToRgb;

impl<C> Function<C> for DistGradToRgb
where
    C: Get<(DistanceF32, GradientF32)>,
{
    type Output = Rgb<f32>;

    fn call(input: C) -> Self::Output {
        let (Distance(dist), Gradient(gx, gy)) = input.get();

        let c = if dist <= 0.0 {
            [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 1.0 - dist]
        } else {
            [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 0.0]
        };

        *Pixel::from_slice(&c)
    }
}

fn main() {
    let viuer = move || {
        DebugMultilineF
            .compose_l(PrintLn)
            /*
            .fanout(
                make_viuer::<Dist, PosDistGrad, PosDistGrad, DistToLuma>(48, 48)
                    .compose_l(ResultUnwrap),
            )
            */
            .fanout(
                make_viuer::<DistGrad<f32>, PosDistGrad<f32>, PosDistGrad<f32>, DistGradToRgb>(
                    48, 48,
                )
                .compose_l(ResultUnwrap),
            )
            .compose_l(Snd)
    };

    let shape_a =
        Do >> Translate(Const(-0.5), Const(-0.5)) >> Circle(Const(1.2)) >> Done << viuer();
    let shape_b = Do >> Translate(Const(0.5), Const(0.5)) >> Circle(Const(1.1)) >> Done << viuer();
    let shape_c = Do >> Translate(Const(0.0), Const(0.5)) >> Circle(Const(1.3)) >> Done << viuer();
    let shape_d =
        Do >> Translate(Const(0.0), Const(-0.5)) >> Circle(Const(1.15)) >> Done << viuer();

    let combined = shape_a * shape_b + shape_c - shape_d << viuer();

    let _shape = Do
        >> Translate(Const(0.25), Const(0.25))
        >> Scale(Const(0.5))
        >> combined
        >> Manifold
        >> Isosurface(Const(0.2))
        >> Done
        << viuer();
}
