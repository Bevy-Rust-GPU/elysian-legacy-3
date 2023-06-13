use t_funk::collection::set::{
    Drop, DropT, Empty, Get, Insert, InsertT, Remove, SubtractFrom, UnionWith,
};

use crate::{Context, ContextRaster, Raster};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextRasterString<C, R, S> {
    pub context_raster: ContextRaster<C, R>,
    pub string: S,
}

impl<C, R, S> Get<Context<C>> for ContextRasterString<Context<C>, R, S> {
    fn get(self) -> Context<C> {
        self.context_raster.get()
    }
}

impl<C, R, S> Get<Raster<R>> for ContextRasterString<C, Raster<R>, S> {
    fn get(self) -> Raster<R> {
        self.context_raster.get()
    }
}

impl<C, R> Get<String> for ContextRasterString<C, R, String> {
    fn get(self) -> String {
        self.string
    }
}

impl<CA, CB, R, S> Insert<Context<CB>> for ContextRasterString<CA, R, S> {
    type Insert = ContextRasterString<Context<CB>, R, S>;

    fn insert(self, t: Context<CB>) -> Self::Insert {
        let ContextRasterString {
            context_raster,
            string,
        } = self;

        ContextRasterString {
            context_raster: context_raster.insert(t),
            string,
        }
    }
}

impl<C, RA, RB, S> Insert<Raster<RB>> for ContextRasterString<C, RA, S> {
    type Insert = ContextRasterString<C, Raster<RB>, S>;

    fn insert(self, t: Raster<RB>) -> Self::Insert {
        let ContextRasterString {
            context_raster,
            string,
        } = self;

        ContextRasterString {
            context_raster: context_raster.insert(t),
            string,
        }
    }
}

impl<C, R, S> Insert<String> for ContextRasterString<C, R, S> {
    type Insert = ContextRasterString<C, R, String>;

    fn insert(self, t: String) -> Self::Insert {
        let ContextRasterString { context_raster, .. } = self;
        ContextRasterString {
            string: t,
            context_raster,
        }
    }
}

impl<C, R, S> Remove<Context<C>> for ContextRasterString<Context<C>, R, S> {
    type Remove = ContextRasterString<(), R, S>;

    fn remove(self) -> (Self::Remove, Context<C>) {
        let ContextRasterString {
            context_raster,
            string,
        } = self;

        let (context_raster, context) = context_raster.remove();

        (
            ContextRasterString {
                context_raster,
                string,
            },
            context,
        )
    }
}

impl<C, R, S> Remove<Raster<R>> for ContextRasterString<C, Raster<R>, S> {
    type Remove = ContextRasterString<C, (), S>;

    fn remove(self) -> (Self::Remove, Raster<R>) {
        let ContextRasterString {
            context_raster,
            string,
        } = self;

        let (context_raster, context) = context_raster.remove();

        (
            ContextRasterString {
                context_raster,
                string,
            },
            context,
        )
    }
}

impl<C, R> Remove<String> for ContextRasterString<C, R, String> {
    type Remove = ContextRasterString<C, R, ()>;

    fn remove(self) -> (Self::Remove, String) {
        let ContextRasterString {
            context_raster,
            string,
        } = self;

        (
            ContextRasterString {
                context_raster,
                string: (),
            },
            string,
        )
    }
}

impl<C, R, S> Empty for ContextRasterString<C, R, S> {
    type Empty = ContextRasterString<(), (), ()>;

    fn empty(self) -> Self::Empty {
        ContextRasterString {
            context_raster: self.context_raster.empty(),
            string: (),
        }
    }
}

impl<C, R, S, U> UnionWith<U> for ContextRasterString<C, R, S>
where
    U: Insert<C>,
    InsertT<U, C>: Insert<R>,
    InsertT<InsertT<U, C>, R>: Insert<S>,
{
    type UnionWith = InsertT<InsertT<InsertT<U, C>, R>, S>;

    fn union_with(self, u: U) -> Self::UnionWith {
        self.context_raster.union_with(u).insert(self.string)
    }
}

impl<C, R, S, U> SubtractFrom<U> for ContextRasterString<C, R, S>
where
    U: Drop<C>,
    DropT<U, C>: Drop<R>,
    DropT<DropT<U, C>, R>: Drop<S>,
{
    type SubtractFrom = DropT<DropT<DropT<U, C>, R>, S>;

    fn subtract_from(self, u: U) -> Self::SubtractFrom {
        u.drop().drop().drop()
    }
}
