use elysian::{make_ascii, Circle, Do, Done, Isosurface, Manifold, Scale, Translate};
use t_funk::closure::Const;

fn main() {
    let ascii = || make_ascii(48, 24);

    let shape_a =
        Do >> Translate(Const(-0.8), Const(-0.8)) >> Circle(Const(0.2)) >> Done << ascii();
    let shape_b = Do >> Translate(Const(0.8), Const(0.8)) >> Circle(Const(0.1)) >> Done << ascii();
    let shape_c = Do >> Translate(Const(0.0), Const(0.8)) >> Circle(Const(0.3)) >> Done << ascii();
    let shape_d =
        Do >> Translate(Const(0.0), Const(-0.8)) >> Circle(Const(0.15)) >> Done << ascii();

    let combined = shape_a + shape_b + shape_c * shape_d << ascii();

    let _shape = Do
        >> Translate(Const(0.25), Const(0.25))
        >> Scale(Const(0.5))
        >> combined
        >> Isosurface(Const(0.2))
        >> Manifold
        >> Done
        << ascii();
}
