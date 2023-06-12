use t_funk::collection::set::{Get, Insert, Remove};

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
