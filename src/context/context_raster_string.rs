use t_funk::collection::set::{Get, Set};

use crate::{Context, ContextRaster, Raster};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextRasterString<C, T> {
    pub context_raster: ContextRaster<C, T>,
    pub string: String,
}

impl<C, T> Get<Context<C>> for ContextRasterString<C, T> {
    fn get(self) -> Context<C> {
        self.context_raster.get()
    }
}

impl<C, T> Get<Raster<T>> for ContextRasterString<C, T> {
    fn get(self) -> Raster<T> {
        self.context_raster.get()
    }
}

impl<C, T> Get<String> for ContextRasterString<C, T> {
    fn get(self) -> String {
        self.string
    }
}

impl<C, T> Set<Context<C>> for ContextRasterString<C, T> {
    fn set(self, t: Context<C>) -> Self {
        ContextRasterString {
            context_raster: self.context_raster.set(t),
            ..self
        }
    }
}

impl<C, T> Set<Raster<T>> for ContextRasterString<C, T> {
    fn set(self, t: Raster<T>) -> Self {
        ContextRasterString {
            context_raster: self.context_raster.set(t),
            ..self
        }
    }
}

impl<C, T> Set<String> for ContextRasterString<C, T> {
    fn set(self, t: String) -> Self {
        ContextRasterString { string: t, ..self }
    }
}
