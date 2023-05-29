use functional_sdf::{make_ascii, shape, Circle, Isosurface, Manifold, Scale, Translate};

fn main() {
    let ascii = || make_ascii(48, 24);

    let shape_a = shape() << Translate(-0.8, -0.8) << Circle(0.2) >> ascii();
    let shape_b = shape() << Translate(0.8, 0.8) << Circle(0.1) >> ascii();
    let shape_c = shape() << Translate(0.0, 0.8) << Circle(0.3) >> ascii();
    let shape_d = shape() << Translate(0.0, -0.8) << Circle(0.15) >> ascii();

    let combined = shape_a + shape_b + shape_c * shape_d >> ascii();

    let _shape =
        shape() << Translate(0.25, 0.25) << Scale(0.5) << combined << Isosurface(0.2) << Manifold
            >> ascii();
}
