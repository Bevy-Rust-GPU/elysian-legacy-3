//! Convert a Raster into an ImageBuffer via mapping function
use core::marker::PhantomData;

use crate::Raster;

use image::{ImageBuffer, Pixel};
use t_funk::{
    closure::{Closure, OutputT},
    macros::Closure,
};

#[derive(Closure)]
pub struct Image<C, F>(PhantomData<(C, F)>);

impl<C, F> Default for Image<C, F> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C, F> Clone for Image<C, F> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<C, F> Copy for Image<C, F> {}

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
