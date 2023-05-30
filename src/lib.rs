extern crate self as elysian;

mod closure;
mod combinator;
mod domain;
mod interpreter;
mod shape;

pub use closure::*;
pub use combinator::*;
pub use domain::*;
pub use interpreter::*;
pub use shape::*;

#[cfg(test)]
mod test {
    use crate::{shape, Domain, Evaluate, Isosurface, Point, Position, Translate};

    use type_fields::t_funk::{
        closure::Compose, function::Id, list::hlist::Nil, CallF, Closure, Either, Fanout, Fst, Lt,
        MakeIf, Split, Transpose,
    };

    #[test]
    fn test_elysian() {
        let shape_a = shape() << Translate(-0.8, -0.8) << Point << Isosurface(0.4);
        let shape_b = shape() << Translate(0.8, 0.8) << Point << Isosurface(0.2);
        let shape_c = shape() << Translate(0.0, 0.8) << Point << Isosurface(0.5);
        let shape_d = shape() << Translate(0.0, -0.8) << Point << Isosurface(0.3);

        let p: Position<f32> = Position(-1.0, 0.0);

        /*
        let foo = (Translate(-1.0, -1.0), Point, Isosurface(2.0), Manifold)
            .to_hlist()
            .fmap(EvaluateF::default())
            .chain();
        let bar = foo.call((p, Nil));
        */

        let oof = Domain::<Evaluate>::domain(shape_a)
            .fanout(Domain::<Evaluate>::domain(shape_b))
            .compose_l(Transpose)
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(Fst.compose_l(Lt).compose_l(Id.fanout(Id))),
            )
            .compose_l(Transpose)
            .compose_l((CallF.compose_l(Either::unwrap)).split(CallF));
        let _rab = oof.call((p, Nil));
        //panic!("{rab:#?}");

        let foo = Domain::<Evaluate>::domain(shape_a);
        let _bar = foo.call((p, Nil));
        //panic!("{bar:#?}");

        let foo = shape_a + shape_b + shape_c * shape_d;
        let bar = Domain::<Evaluate>::domain(foo);
        let _baz = bar.call((p, Nil));
        //panic!("{baz:#?}");
        //

        //let baz = baz.call(baz);

        //panic!("{baz:#?}");
    }
}
