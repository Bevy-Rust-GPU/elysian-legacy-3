use t_funk::collection::set::{Get, Insert, Remove};

use crate::Raster;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<C>(pub C);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextRaster<C, R> {
    pub context: C,
    pub raster: R,
}

impl<C, R> Get<Context<C>> for ContextRaster<Context<C>, R> {
    fn get(self) -> Context<C> {
        self.context
    }
}

impl<C, R> Get<Raster<R>> for ContextRaster<C, Raster<R>> {
    fn get(self) -> Raster<R> {
        self.raster
    }
}

impl<CA, CB, R> Insert<Context<CB>> for ContextRaster<CA, R> {
    type Insert = ContextRaster<Context<CB>, R>;

    fn insert(self, context: Context<CB>) -> Self::Insert {
        let ContextRaster { raster, .. } = self;
        ContextRaster { context, raster }
    }
}

impl<C, RA, RB> Insert<Raster<RB>> for ContextRaster<C, RA> {
    type Insert = ContextRaster<C, Raster<RB>>;

    fn insert(self, raster: Raster<RB>) -> Self::Insert {
        let ContextRaster { context, .. } = self;
        ContextRaster { context, raster }
    }
}

impl<C, R> Remove<Context<C>> for ContextRaster<Context<C>, R> {
    type Remove = ContextRaster<(), R>;

    fn remove(self) -> (Self::Remove, Context<C>) {
        let ContextRaster { context, raster } = self;
        (
            ContextRaster {
                context: (),
                raster,
            },
            context,
        )
    }
}

impl<C, R> Remove<Raster<R>> for ContextRaster<C, Raster<R>> {
    type Remove = ContextRaster<C, ()>;

    fn remove(self) -> (Self::Remove, Raster<R>) {
        let ContextRaster { context, raster } = self;
        (
            ContextRaster {
                context,
                raster: (),
            },
            raster,
        )
    }
}
