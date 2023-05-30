//! Rasterize a shape into an array of domain outputs

use core::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::{
    root_shape, Domain, DomainT, Domains, DomainsT, PosDistGrad, Position, PositionF32, RootShape,
};

use type_fields::t_funk::{
    arrow::Fanout,
    closure::OutputT,
    function::Id,
    set::Set,
    CallF, Closure,
    closure::{Compose, Composed},
    Copointed, FanoutT, Fmap, Pointed,
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

pub struct Rasterize<D, C> {
    pub width: usize,
    pub height: usize,
    pub phantom: PhantomData<(D, C)>,
}

impl<D, C> Default for Rasterize<D, C> {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            phantom: PhantomData,
        }
    }
}

impl<D, C> Clone for Rasterize<D, C> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            phantom: self.phantom,
        }
    }
}

impl<D, C> Copy for Rasterize<D, C> {}

impl<D, C, S> Closure<S> for Rasterize<D, C>
where
    S: Domains<D>,
    DomainsT<S, D>: Fanout<Id>,
    Composed<CallF, FanoutT<DomainsT<S, D>, Id>>: Clone + Closure<C>,
    OutputT<Composed<CallF, FanoutT<DomainsT<S, D>, Id>>, C>: Clone + Default,
    C: Default + Set<PositionF32>,
{
    type Output = Raster<OutputT<Composed<CallF, FanoutT<DomainsT<S, D>, Id>>, C>>;

    fn call(self, shape: S) -> Self::Output {
        let func = shape.domains().fanout(Id).compose_l(CallF);

        let mut out: Self::Output = Raster::new(self.width, self.height);
        for (y, row) in out.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let nx = ((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / self.height as f32) * 2.0 - 1.0;
                *col = func.clone().call(C::default().set(Position(nx, ny)));
            }
        }
        out
    }
}
