//! Rasterize a shape into an array of domain outputs

use std::ops::{Deref, DerefMut};

use t_funk::{
    closure::Closure, closure::OutputT, typeclass::copointed::Copointed, typeclass::functor::Fmap,
    typeclass::pointed::Pointed,
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
