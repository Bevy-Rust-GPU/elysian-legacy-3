use t_funk::collection::set::{
    Drop, DropT, Empty, Get, Insert, InsertT, Remove, SubtractFrom, UnionWith,
};

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

impl<C, R> Empty for ContextRaster<C, R> {
    type Empty = ContextRaster<(), ()>;

    fn empty(self) -> Self::Empty {
        ContextRaster {
            context: (),
            raster: (),
        }
    }
}

impl<C, R, U> UnionWith<U> for ContextRaster<C, R>
where
    U: Insert<C>,
    InsertT<U, C>: Insert<R>,
{
    type UnionWith = InsertT<InsertT<U, C>, R>;

    fn union_with(self, u: U) -> Self::UnionWith {
        u.insert(self.context).insert(self.raster)
    }
}

impl<C, R, U> SubtractFrom<U> for ContextRaster<C, R>
where
    U: Drop<C>,
    DropT<U, C>: Drop<R>,
{
    type SubtractFrom = DropT<DropT<U, C>, R>;

    fn subtract_from(self, u: U) -> Self::SubtractFrom {
        u.drop().drop()
    }
}
