use elysian::{
    adt, intersection, make_ascii, union, Circle, Done, Isosurface, Manifold, Scale, Translate,
};
use glam::Vec2;
use t_funk::op_chain::tap;

fn main() {
    let ascii = || make_ascii(48, 24);

    let shape_a =
        adt() << Translate(Vec2::new(0.8, -0.8)) << Circle(0.2_f32) >> tap(ascii()) >> Done;
    let shape_b =
        adt() << Translate(Vec2::new(0.8, 0.8)) << Circle(0.1_f32) >> tap(ascii()) >> Done;
    let shape_c =
        adt() << Translate(Vec2::new(0.0, 0.8)) << Circle(0.3_f32) >> tap(ascii()) >> Done;
    let shape_d =
        adt() << Translate(Vec2::new(0.0, -0.8)) << Circle(0.15_f32) >> tap(ascii()) >> Done;

    let combined = union() << shape_a << shape_b << shape_c >> intersection() << shape_d
        >> tap(ascii())
        >> Done;

    let _shape = adt()
        << Translate(Vec2::new(0.25, 0.25))
        << Scale(0.5_f32)
        << combined
        << Isosurface(0.2_f32)
        << Manifold
        >> tap(ascii())
        >> Done;
}
