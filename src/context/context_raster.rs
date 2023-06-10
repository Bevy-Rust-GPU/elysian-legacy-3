use t_funk::collection::set::{Get, Set};

use crate::Raster;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<C>(pub C);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextRaster<C, T> {
    pub context: Context<C>,
    pub raster: Raster<T>,
}

impl<C, T> Get<Context<C>> for ContextRaster<C, T> {
    fn get(self) -> Context<C> {
        self.context
    }
}

impl<C, T> Get<Raster<T>> for ContextRaster<C, T> {
    fn get(self) -> Raster<T> {
        self.raster
    }
}

impl<C, T> Set<Context<C>> for ContextRaster<C, T> {
    fn set(self, t: Context<C>) -> Self {
        Self { context: t, ..self }
    }
}

impl<C, T> Set<Raster<T>> for ContextRaster<C, T> {
    fn set(self, t: Raster<T>) -> Self {
        Self { raster: t, ..self }
    }
}
