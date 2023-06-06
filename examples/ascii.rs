use elysian::{
    intersection, make_ascii, shape, union, Circle, Done, Isosurface, Manifold, Scale, Translate,
};
use t_funk::{closure::Const, r#do::tap};

fn main() {
    let ascii = || make_ascii(48, 24);

    let shape_a = shape() << Translate(Const(-0.8), Const(-0.8)) << Circle(Const(0.2))
        >> tap(ascii())
        >> Done;
    let shape_b =
        shape() << Translate(Const(0.8), Const(0.8)) << Circle(Const(0.1)) >> tap(ascii()) >> Done;
    let shape_c =
        shape() << Translate(Const(0.0), Const(0.8)) << Circle(Const(0.3)) >> tap(ascii()) >> Done;
    let shape_d = shape() << Translate(Const(0.0), Const(-0.8)) << Circle(Const(0.15))
        >> tap(ascii())
        >> Done;

    let combined = union() << shape_a << shape_b << shape_c >> intersection() << shape_d
        >> tap(ascii())
        >> Done;

    let _shape = shape()
        << Translate(Const(0.25), Const(0.25))
        << Scale(Const(0.5))
        << combined
        << Isosurface(Const(0.2))
        << Manifold
        >> tap(ascii())
        >> Done;
}
