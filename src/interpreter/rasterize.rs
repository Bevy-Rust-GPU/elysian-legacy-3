use core::marker::PhantomData;

use crate::{Domain, Position, PositionF32, Evaluate, EvaluateT};

use type_fields::t_funk::{closure::Compose, hlist::Nil, Closure, Composed, Curry2, Fst, Tuple};

pub type Raster<const W: usize, const H: usize, T> = [[T; W]; H];
pub type RasterF32<const W: usize, const H: usize> = Raster<W, H, f32>;
pub type RasterRGB32<const W: usize, const H: usize> = Raster<W, H, (f32, f32, f32)>;
pub type RasterU8<const W: usize, const H: usize> = Raster<W, H, u8>;
pub type RasterRGB8<const W: usize, const H: usize> = Raster<W, H, (u8, u8, u8)>;

pub fn raster<const W: usize, const H: usize, T>(t: T) -> Raster<W, H, T>
where
    T: Copy,
{
    [[t; W]; H]
}

pub struct Rasterize<const EX: usize, const EY: usize, T>(PhantomData<T>);

impl<const EX: usize, const EY: usize, T> Default for Rasterize<EX, EY, T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<const EX: usize, const EY: usize, T> Clone for Rasterize<EX, EY, T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<const EX: usize, const EY: usize, T> Copy for Rasterize<EX, EY, T> {}

impl<const EX: usize, const EY: usize, T, S> Closure<S> for Rasterize<EX, EY, T>
where
    T: Default + Copy,
    S: Clone,
    S: Domain<Evaluate>,
    Fst: Compose<EvaluateT<S>>,
    Composed<Fst, EvaluateT<S>>: Clone + Closure<(PositionF32, Nil), Output = T>,
{
    type Output = Raster<EX, EY, T>;

    fn call(self, shape: S) -> Self::Output {
        let mut out: Self::Output = raster(Default::default());

        let func = shape
            .clone()
            .domain()
            .compose_l(Fst)
            .compose(Curry2::suffix2(Tuple, Nil));

        for y in 0..EY {
            for x in 0..EX {
                let nx = ((x as f32 + 0.5) / EX as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / EY as f32) * 2.0 - 1.0;
                let dist = func.clone().call(Position(nx, ny));
                out[y][x] = dist;
            }
        }

        out
    }
}
