use core::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{
    Distance, DistanceF, DistanceF32, Domain, Gradient, GradientF, GradientF32, Position,
    PositionF32, Evaluate, EvaluateT,
};

use image::{ImageBuffer, Luma, Pixel, Rgb};
use type_fields::{
    macros::Closure,
    t_funk::{
        arrow::Second, closure::Compose, hlist::Nil, list::hlist::ChainF, CallF, Closure, Composed,
        Curry2, Curry2B, Fanout, Fanouted, FlipTuple, FmapF, Fst, RShiftTuple, Seconded, Snd,
        Tuple,
    },
};

#[derive(Closure)]
pub struct Image<D>(PhantomData<D>);

impl<D> Default for Image<D> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<D> Clone for Image<D> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<D> Copy for Image<D> {}

impl<C, S> Closure<(ImageBuffer<Luma<f32>, C>, S)> for Image<DistanceF>
where
    C: Deref<Target = [f32]> + DerefMut,
    S: Domain<Evaluate>,
    Fst: Compose<EvaluateT<S>>,
    Composed<Fst, EvaluateT<S>>: Clone + Closure<(PositionF32, Nil), Output = DistanceF32>,
{
    type Output = ImageBuffer<Luma<f32>, C>;

    fn call(self, (mut buf, shape): (ImageBuffer<Luma<f32>, C>, S)) -> Self::Output {
        let func = shape
            .domain()
            .compose_l(Fst)
            .compose(Curry2::suffix2(Tuple, Nil));

        let (w, h) = buf.dimensions();

        for y in 0..h {
            for x in 0..w {
                let nx = ((x as f32 + 0.5) / w as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / h as f32) * 2.0 - 1.0;

                let dist = 1.0 - func.clone().call(Position(nx, ny)).0.max(0.0).min(1.0);

                buf.put_pixel(x, y, *Pixel::from_slice(&[dist]));
            }
        }

        buf
    }
}

impl<C, S> Closure<(ImageBuffer<Rgb<f32>, C>, S)> for Image<GradientF>
where
    C: Deref<Target = [f32]> + DerefMut,
    S: Domain<Evaluate>,
    Composed<
        Fanouted<Composed<Snd, Snd>, Composed<Snd, CallF>>,
        Composed<
            FlipTuple,
            Composed<
                RShiftTuple,
                Fanouted<
                    Fst,
                    Composed<
                        Seconded<
                            Composed<type_fields::t_funk::hlist::ChainF, Curry2B<FmapF, GradientF>>,
                        >,
                        EvaluateT<S>,
                    >,
                >,
            >,
        >,
    >: Clone + Closure<(Position<f32>, Nil), Output = (DistanceF32, GradientF32)>,
{
    type Output = ImageBuffer<Rgb<f32>, C>;

    fn call(self, (mut buf, shape): (ImageBuffer<Rgb<f32>, C>, S)) -> Self::Output {
        let func = Fst
            .fanout(
                shape.domain().compose_l(
                    FmapF
                        .suffix2(GradientF::default())
                        .compose_l(ChainF)
                        .second(),
                ),
            )
            .compose_l(RShiftTuple)
            .compose_l(FlipTuple)
            .compose_l(Snd.compose_l(Snd).fanout(CallF.compose_l(Snd)));

        let (w, h) = buf.dimensions();

        for y in 0..h {
            for x in 0..w {
                let nx = ((x as f32 + 0.5) / w as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / h as f32) * 2.0 - 1.0;

                let (Distance(dist), Gradient(gx, gy)) = func.clone().call((Position(nx, ny), Nil));

                let col = if dist <= 0.0 {
                    [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 1.0 - dist]
                } else {
                    [gx * 0.5 + 0.5, gy * 0.5 + 0.5, 0.0]
                };

                buf.put_pixel(x, y, *Pixel::from_slice(&col));
            }
        }

        buf
    }
}
