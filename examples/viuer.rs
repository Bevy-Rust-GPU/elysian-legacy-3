use functional_sdf::{
    make_viuer, shape, Circle, GradientF, Isosurface, Manifold, Scale, Translate, DistanceF,
};
use image::{Rgb, Luma};
use type_fields::t_funk::{
    closure::Compose, DebugMultilineF, Fanout, Function, PrintLn, ResultUnwrap, Snd,
};

fn main() {
    let viuer = move || {
        DebugMultilineF
            .compose_l(PrintLn)
            //.fanout(make_viuer::<DistanceF, Luma<f32>>(48, 48).compose_l(ResultUnwrap))
            .fanout(make_viuer::<GradientF, Rgb<f32>>(48, 48).compose_l(ResultUnwrap))
            .compose_l(Snd)
    };

    let shape_a = shape() << Translate(-0.5, -0.5) << Circle(1.2) >> viuer();
    let shape_b = shape() << Translate(0.5, 0.5) << Circle(1.1) >> viuer();
    let shape_c = shape() << Translate(0.0, 0.5) << Circle(1.3) >> viuer();
    let shape_d = shape() << Translate(0.0, -0.5) << Circle(1.15) >> viuer();

    let combined = shape_a * shape_b + shape_c - shape_d >> viuer();

    let _shape =
        shape() << Translate(0.25, 0.25) << Scale(0.5) << combined << Manifold << Isosurface(0.1)
            >> viuer();
}
