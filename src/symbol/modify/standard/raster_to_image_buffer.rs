use std::marker::PhantomData;

use crate::glam::{Vec2, Vec3, Vec4};
use image::{ImageBuffer, Luma, Pixel, Rgb, Rgba};
use t_funk::{
    closure::{Closure, OutputT},
    collection::set::Get,
    macros::{lift, Closure},
    typeclass::{
        copointed::Copointed,
        functor::Fmap,
        semigroup::{Mappend, MappendT}, monad::Identity,
    },
};

use crate::{
    Color, Distance, EvaluateFunction, EvaluateInputs, Gradient, IntoMonad, IntoTuple, IntoTupleT,
    Invert, LiftAdt, Modify, Raster, Saturate,
};

pub trait ToImage<F>: Sized + IntoTuple {
    type ToImage<R>
    where
        RasterToImage<R, F>: IntoTuple,
        IntoTupleT<Self>: Mappend<IntoTupleT<RasterToImage<R, F>>>;

    fn to_image<R>(self, f: F) -> Self::ToImage<R>
    where
        RasterToImage<R, F>: IntoTuple,
        IntoTupleT<Self>: Mappend<IntoTupleT<RasterToImage<R, F>>>;
}

impl<T, F> ToImage<F> for T
where
    T: Sized + IntoTuple,
{
    type ToImage<R> = MappendT<IntoTupleT<T>, IntoTupleT<RasterToImage<R, F>>>
        where
            RasterToImage<R, F>: IntoTuple,
            IntoTupleT<T>: Mappend<IntoTupleT<RasterToImage<R, F>>>;

    fn to_image<R>(self, f: F) -> Self::ToImage<R>
    where
        RasterToImage<R, F>: IntoTuple,
        IntoTupleT<T>: Mappend<IntoTupleT<RasterToImage<R, F>>>,
    {
        self.into_tuple()
            .mappend(RasterToImage(f, PhantomData).into_tuple())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterToImage<R, F>(pub F, pub PhantomData<R>);

impl<R, G, F> Fmap<F> for RasterToImage<R, G> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<R, F> IntoMonad for RasterToImage<R, F> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<R, F> LiftAdt for RasterToImage<R, F> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<R, F, D> EvaluateInputs<D> for RasterToImage<R, F> {
    type Inputs = Raster<R>;
    type Moves = Raster<R>;
}

impl<R, F, D> EvaluateFunction<D> for RasterToImage<R, F> {
    type Function = Image<R, F>;

    fn evaluate_function(self) -> Self::Function {
        Image(self.0, self.1)
    }
}

#[lift]
pub fn dist_to_luma<C>(c: C) -> Luma<f32>
where
    C: Get<Distance<f32>>,
{
    *Pixel::from_slice(&[c.get().fmap(Saturate).fmap(Invert).copoint()])
}

#[lift]
pub fn dist_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<Distance<f32>>,
{
    let c = c.get().fmap(Saturate).fmap(Invert).copoint();
    *Pixel::from_slice(&[c, c, c])
}

#[lift]
pub fn dist_to_rgba<C>(k: f32, c: C) -> Rgba<f32>
where
    C: Get<Distance<f32>>,
{
    let c = c.get().fmap(Saturate).fmap(Invert).copoint() * k;
    *Pixel::from_slice(&[c, c, c, 1.0])
}

#[lift]
pub fn dist_grad_to_rgb<C>(c: C) -> Rgb<f32>
where
    C: Get<(Distance<f32>, Gradient<Vec2>)>,
{
    let (Distance(dist), Gradient(g)) = c.get();

    let g = g * 0.5 + 0.5;
    let c = g.extend(if dist <= 0.0 { 1.0 - dist } else { 0.0 });

    let c = [c.x, c.y, c.z];

    *Pixel::from_slice(&c)
}

#[lift]
pub fn dist_grad_to_rgba<C>(c: C) -> Rgba<f32>
where
    C: Get<(Distance<f32>, Gradient<Vec2>)>,
{
    let (Distance(dist), Gradient(g)) = c.get();

    let g = g * 0.5 + 0.5;
    let c = g.extend(if dist <= 0.0 { 1.0 - dist } else { 0.0 });

    let c = [c.x, c.y, c.z, 1.0];

    *Pixel::from_slice(&c)
}

#[lift]
pub fn dist_color_to_rgb<C>((clear, k): (Vec3, f32), ctx: C) -> Rgb<f32>
where
    C: Get<(Distance<f32>, Color<Vec3>)>,
{
    let (Distance(dist), Color(color)) = ctx.get();

    let l = (-dist * k).max(0.0).min(1.0);
    let c = clear.lerp(color, l);

    let c = [c.x, c.y, c.z];

    *Pixel::from_slice(&c)
}

#[lift]
pub fn color_to_rgb<C>(ctx: C) -> Rgb<f32>
where
    C: Get<Color<Vec3>>,
{
    let Color(c) = ctx.get();

    let c = [c.x, c.y, c.z];

    *Pixel::from_slice(&c)
}

#[lift]
pub fn color_to_rgba<C>(ctx: C) -> Rgba<f32>
where
    C: Get<Color<Vec4>>,
{
    let Color(c) = ctx.get();

    let c = [c.x, c.y, c.z, c.w];

    *Pixel::from_slice(&c)
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Image<C, F>(pub F, pub PhantomData<C>);

impl<C, F> Closure<Raster<C>> for Image<C, F>
where
    C: Clone,
    F: Clone + Closure<C>,
    OutputT<F, C>: Pixel,
{
    type Output = ImageBuffer<OutputT<F, C>, Vec<<OutputT<F, C> as Pixel>::Subpixel>>;

    fn call(self, rast: Raster<C>) -> Self::Output {
        let w = rast[0].len() as u32;
        let h = rast.len() as u32;

        let mut buf = ImageBuffer::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let dist = self.0.clone().call(rast[y as usize][x as usize].clone());
                buf.put_pixel(x, y, dist);
            }
        }

        buf
    }
}
