mod combinator;
mod domain;
mod interpreter;
mod shape;

pub use combinator::*;
pub use domain::*;
pub use interpreter::*;
pub use shape::*;

#[cfg(test)]
mod test {
    use crate::{
        shape, Ascii, Distance, DistanceF, DistanceF32, Domain, Gradient, GradientF, GradientF32,
        Image, Isosurface, Manifold, Point, PointDistance, Position, Evaluate, EvaluateF, Translate,
        ViuerPrint, ASCII_RAMP,
    };

    use image::{DynamicImage, ImageBuffer};
    use type_fields::t_funk::{
        arrow::First,
        arrow::Second,
        closure::Compose,
        function::{Const, Id},
        hlist::{Chain, PushBackF},
        list::hlist::ChainF,
        list::hlist::Nil,
        tlist::ToHList,
        CallF, Closure, Composed, CopointF, Curry2, Curry2B, Either, Fanout, Fanouted, FlipTuple,
        Fmap, FmapF, Fst, IntoF, Lt, MakeIf, PrintLn, RShiftTuple, Seconded, Snd, Split, Splitted,
        Transpose,
    };
    use viuer::Config;

    #[test]
    fn test_functional_sdf() {
        let shape_a = shape() << Translate(-0.8, -0.8) << Point << Isosurface(0.4);
        let shape_b = shape() << Translate(0.8, 0.8) << Point << Isosurface(0.2);
        let shape_c = shape() << Translate(0.0, 0.8) << Point << Isosurface(0.5);
        let shape_d = shape() << Translate(0.0, -0.8) << Point << Isosurface(0.3);

        let p: Position<f32> = Position(-1.0, 0.0);

        let foo = (Translate(-1.0, -1.0), Point, Isosurface(2.0), Manifold)
            .to_hlist()
            .fmap(EvaluateF::default())
            .chain();
        let bar = foo.call((p, Nil));

        let oof = Domain::<Evaluate>::domain(shape_a)
            .fanout(Domain::<Evaluate>::domain(shape_b))
            .compose_l(Transpose)
            .compose_l(CopointF.split(CopointF).first())
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(Fst.compose_l(Lt).compose_l(Id.fanout(Id))),
            )
            .compose_l(Transpose)
            .compose_l((CallF.compose_l(Either::unwrap)).split(CallF));
        let rab = oof.call((p, Nil));
        panic!("{rab:#?}");

        let foo = Domain::<Evaluate>::domain(shape_a);
        let _bar = foo.call((p, Nil));
        //panic!("{bar:#?}");

        let foo = shape_a + shape_b + shape_c * shape_d;
        let bar = Domain::<Evaluate>::domain(foo);
        let _baz = bar.call((p, Nil));
        //panic!("{baz:#?}");
        //

        let foo = shape() << Point << Manifold;
        let bar = Fst
            .fanout(
                Domain::<Evaluate>::domain(foo).compose_l(
                    FmapF
                        .suffix2(GradientF::default())
                        .compose_l(ChainF)
                        .second(),
                ),
            )
            .compose_l(RShiftTuple)
            .compose_l(FlipTuple)
            .compose_l(Snd.compose_l(Snd).fanout(CallF.compose_l(Snd)));
        //panic!("{bar:#?}");
        let baz = bar.call((p, Nil));
        panic!("{baz:#?}");

        //let baz = baz.call(baz);

        //panic!("{baz:#?}");

        let ascii = Ascii::<64, 32, f32>::default().curry2().call(ASCII_RAMP);
        let _out = ascii.call(foo);
        //panic!("\n{out:}");

        let config = Config {
            transparent: false,
            absolute_offset: false,
            x: 0,
            y: 0,
            restore_cursor: false,
            width: Some(48),
            height: None,
            truecolor: true,
            use_kitty: true,
            use_iterm: false,
        };

        let viuer = Image::<DistanceF>::default()
            .curry2()
            .call(ImageBuffer::new(48, 48))
            .compose_l(IntoF::<DynamicImage>::default())
            .compose_l(ViuerPrint.curry2().call(config))
            .compose(
                Id.fanout(Const.prefix2(String::default()).compose_l(PrintLn))
                    .compose_l(Fst),
            );

        viuer.call(foo).unwrap();
    }
}
