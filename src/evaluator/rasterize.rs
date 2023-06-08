//! Rasterize a shape into an array of domain outputs

use core::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{Evaluate, EvaluateT, Position};

use glam::Vec2;
use t_funk::{
    closure::Closure, closure::OutputT, collection::set::Set, typeclass::copointed::Copointed,
    typeclass::functor::Fmap, typeclass::pointed::Pointed,
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raster<T>(pub Vec<Vec<T>>);

impl<T> Raster<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Raster(Vec::from_iter(
            std::iter::repeat(Vec::from_iter(
                std::iter::repeat(Default::default()).take(width),
            ))
            .take(height),
        ))
    }
}

impl<T> Pointed for Raster<T> {
    type Pointed = Vec<Vec<T>>;

    fn point(unit: Self::Pointed) -> Self {
        Raster(unit)
    }
}

impl<T> Copointed for Raster<T> {
    type Copointed = Vec<Vec<T>>;

    fn copoint(self) -> Self::Copointed {
        self.0
    }
}

impl<T> Deref for Raster<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Raster<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, F> Fmap<F> for Raster<T>
where
    T: Clone,
    F: Clone + Closure<T>,
    OutputT<F, T>: Default + Copy,
{
    type Fmap = Raster<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Raster(
            self.0
                .into_iter()
                .map(|row| row.into_iter().map(|col| f.clone().call(col)).collect())
                .collect(),
        )
    }
}

pub type RasterF32<const W: usize, const H: usize> = Raster<f32>;
pub type RasterRGB32<const W: usize, const H: usize> = Raster<(f32, f32, f32)>;
pub type RasterU8<const W: usize, const H: usize> = Raster<u8>;
pub type RasterRGB8<const W: usize, const H: usize> = Raster<(u8, u8, u8)>;

pub struct Rasterize<D> {
    pub width: usize,
    pub height: usize,
    pub phantom: PhantomData<D>,
}

impl<D> Default for Rasterize<D> {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            phantom: PhantomData,
        }
    }
}

impl<D> Clone for Rasterize<D> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            phantom: self.phantom,
        }
    }
}

impl<D> Copy for Rasterize<D> {}

impl<D, C, S> Closure<(C, S)> for Rasterize<D>
where
    S: Clone + Evaluate<D, C>,
    EvaluateT<S, D, C>: Default + Clone,
    C: Clone + Set<Position<Vec2>>,
{
    type Output = Raster<EvaluateT<S, D, C>>;

    fn call(self, (ctx, shape): (C, S)) -> Self::Output {
        let mut out: Self::Output = Raster::new(self.width, self.height);
        for (y, row) in out.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let nx = ((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / self.height as f32) * 2.0 - 1.0;
                *col = Evaluate::<D, C>::evaluate(
                    shape.clone(),
                    ctx.clone().set(Position(Vec2::new(nx, ny))),
                );
            }
        }
        out
    }
}
