//! Convert a Raster into an ImageBuffer via mapping function
use core::marker::PhantomData;

use crate::Raster;

use image::{ImageBuffer, Pixel};
use type_fields::{
    macros::Closure,
    t_funk::{closure::OutputT, Closure},
};

#[derive(Closure)]
pub struct Image<D, F>(PhantomData<(D, F)>);

impl<D, F> Default for Image<D, F> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<D, F> Clone for Image<D, F> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<D, F> Copy for Image<D, F> {}

impl<D, F> Closure<Raster<D>> for Image<D, F>
where
    D: Clone,
    F: Default + Closure<D>,
    OutputT<F, D>: Pixel,
{
    type Output = ImageBuffer<OutputT<F, D>, Vec<<OutputT<F, D> as Pixel>::Subpixel>>;

    fn call(self, rast: Raster<D>) -> Self::Output {
        let w = rast[0].len() as u32;
        let h = rast.len() as u32;

        let mut buf = ImageBuffer::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let dist = F::default().call(rast[y as usize][x as usize].clone());
                buf.put_pixel(x, y, dist);
            }
        }

        buf
    }
}
