use functional_sdf::{shape, Ascii, Circle, Isosurface, Manifold, Scale, Translate, ASCII_RAMP};

use type_fields::t_funk::{
    closure::Compose,
    function::{Const, Id},
    Closure, Curry2, Fanout, Fst, PrintLn,
};

fn main() {
    let ascii = move || {
        Ascii::<48, 24, f32>::default()
            .curry2()
            .call(ASCII_RAMP)
            .compose_l(PrintLn)
            .compose(
                Id.fanout(Const.prefix2(String::default()).compose_l(PrintLn))
                    .compose_l(Fst),
            )
    };

    let shape_a = shape() << Translate(-0.8, -0.8) << Circle(0.2) >> ascii();
    let shape_b = shape() << Translate(0.8, 0.8) << Circle(0.1) >> ascii();
    let shape_c = shape() << Translate(0.0, 0.8) << Circle(0.3) >> ascii();
    let shape_d = shape() << Translate(0.0, -0.8) << Circle(0.15) >> ascii();

    let combined = shape_a + shape_b + shape_c * shape_d;

    let _shape =
        shape() << Translate(0.25, 0.25) << Scale(0.5) << combined << Isosurface(0.2) << Manifold
            >> ascii();
}
