use std::ops::Deref;

use image::{ImageBuffer, Pixel};
use t_funk::collection::set::{Get, Set};

use crate::{Context, ContextRaster, Raster};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    pub context_raster: ContextRaster<C, T>,
    pub image: ImageBuffer<P, PC>,
}

impl<C, T, P, PC> Default for ContextRasterImage<C, T, P, PC>
where
    C: Default,
    T: Default,
    P: Pixel,
    PC: Default,
{
    fn default() -> Self {
        Self {
            context_raster: Default::default(),
            image: Default::default(),
        }
    }
}

impl<C, T, P, PC> Clone for ContextRasterImage<C, T, P, PC>
where
    C: Clone,
    T: Clone,
    P: Pixel,
    PC: Clone,
    PC: Deref<Target = [P::Subpixel]>,
{
    fn clone(&self) -> Self {
        Self {
            context_raster: self.context_raster.clone(),
            image: self.image.clone(),
        }
    }
}

impl<C, T, P, PC> Get<Context<C>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn get(self) -> Context<C> {
        self.context_raster.get()
    }
}

impl<C, T, P, PC> Get<Raster<T>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn get(self) -> Raster<T> {
        self.context_raster.get()
    }
}

impl<C, T, P, PC> Get<ImageBuffer<P, PC>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn get(self) -> ImageBuffer<P, PC> {
        self.image
    }
}

impl<C, T, P, PC> Set<Context<C>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn set(self, t: Context<C>) -> Self {
        ContextRasterImage {
            context_raster: self.context_raster.set(t),
            ..self
        }
    }
}

impl<C, T, P, PC> Set<Raster<T>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn set(self, t: Raster<T>) -> Self {
        ContextRasterImage {
            context_raster: self.context_raster.set(t),
            ..self
        }
    }
}

impl<C, T, P, PC> Set<ImageBuffer<P, PC>> for ContextRasterImage<C, T, P, PC>
where
    P: Pixel,
{
    fn set(self, t: ImageBuffer<P, PC>) -> Self {
        ContextRasterImage { image: t, ..self }
    }
}
