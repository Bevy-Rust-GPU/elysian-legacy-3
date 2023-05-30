use elysian::{
    make_viuer, shape, Circle, Distance, DistanceF32, Gradient, GradientF32, Invert, Isosurface,
    Manifold, PosDistGrad, Saturate, Scale, Translate,
};
use image::{Luma, Pixel, Rgb};
use type_fields::{
    macros::Closure,
    t_funk::{
        closure::Compose, Copointed, DebugMultilineF, Fanout, Fmap, Function, PrintLn,
        ResultUnwrap, Snd,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct DistToLuma;

impl Function<DistanceF32> for DistToLuma {
    type Output = Luma<f32>;

    fn call(input: DistanceF32) -> Self::Output {
        *Pixel::from_slice(&[input.fmap(Saturate).fmap(Invert).copoint()])
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
struct DistGradToRgb;

impl Function<(DistanceF32, GradientF32)> for DistGradToRgb {
    type Output = Rgb<f32>;

    fn call((Distance(dist), Gradient(gx, gy)): (DistanceF32, GradientF32)) -> Self::Output {
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
            .fanout(
                make_viuer::<(DistanceF32, ()), PosDistGrad, DistanceF32, DistToLuma>(48, 48)
                    .compose_l(ResultUnwrap),
            )
            /*
            .fanout(
                make_viuer::<
                    Split<DistanceF32, GradientF32>,
                    (DistanceF32, GradientF32),
                    DistGradToRgb,
                >(48, 48)
                .compose_l(ResultUnwrap),
            )
            */
            .compose_l(Snd)
    };

    // FIXME
    /*
    let shape_a = shape() << Translate(-0.5, -0.5) << Circle(1.2) >> viuer();
    let shape_b = shape() << Translate(0.5, 0.5) << Circle(1.1) >> viuer();
    let shape_c = shape() << Translate(0.0, 0.5) << Circle(1.3) >> viuer();
    let shape_d = shape() << Translate(0.0, -0.5) << Circle(1.15) >> viuer();

    let combined = shape_a * shape_b + shape_c - shape_d >> viuer();

    let _shape =
        shape() << Translate(0.25, 0.25) << Scale(0.5) << combined << Manifold << Isosurface(0.2)
            >> viuer();
    */
}
