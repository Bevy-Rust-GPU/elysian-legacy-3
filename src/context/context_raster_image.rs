use image::{ImageBuffer, Pixel};
use t_funk::collection::set::{
    Drop, DropT, Empty, Get, Insert, InsertT, Remove, SubtractFrom, UnionWith,
};

use crate::{Context, ContextRaster, Raster};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ContextRasterImage<C, R, I> {
    pub context_raster: ContextRaster<C, R>,
    pub image: I,
}

impl<C, R, I> Default for ContextRasterImage<C, R, I>
where
    C: Default,
    R: Default,
    I: Default,
{
    fn default() -> Self {
        Self {
            context_raster: Default::default(),
            image: Default::default(),
        }
    }
}

impl<C, R, I> Clone for ContextRasterImage<C, R, I>
where
    C: Clone,
    R: Clone,
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            context_raster: self.context_raster.clone(),
            image: self.image.clone(),
        }
    }
}

impl<C, R, I> Get<Context<C>> for ContextRasterImage<Context<C>, R, I> {
    fn get(self) -> Context<C> {
        self.context_raster.get()
    }
}

impl<C, R, I> Get<Raster<R>> for ContextRasterImage<C, Raster<R>, I> {
    fn get(self) -> Raster<R> {
        self.context_raster.get()
    }
}

impl<C, R, P, PC> Get<ImageBuffer<P, PC>> for ContextRasterImage<C, R, ImageBuffer<P, PC>>
where
    P: Pixel,
{
    fn get(self) -> ImageBuffer<P, PC> {
        self.image
    }
}

impl<CA, CB, R, I> Insert<Context<CB>> for ContextRasterImage<CA, R, I> {
    type Insert = ContextRasterImage<Context<CB>, R, I>;

    fn insert(self, t: Context<CB>) -> Self::Insert {
        let ContextRasterImage {
            context_raster,
            image,
        } = self;

        ContextRasterImage {
            context_raster: context_raster.insert(t),
            image,
        }
    }
}

impl<C, RA, RB, I> Insert<Raster<RB>> for ContextRasterImage<C, RA, I> {
    type Insert = ContextRasterImage<C, Raster<RB>, I>;

    fn insert(self, t: Raster<RB>) -> Self::Insert {
        let ContextRasterImage {
            context_raster,
            image,
        } = self;

        ContextRasterImage {
            context_raster: context_raster.insert(t),
            image,
        }
    }
}

impl<C, R, PA, PB, PCA, PCB> Insert<ImageBuffer<PB, PCB>>
    for ContextRasterImage<C, R, ImageBuffer<PA, PCA>>
where
    PA: Pixel,
    PB: Pixel,
{
    type Insert = ContextRasterImage<C, R, ImageBuffer<PB, PCB>>;

    fn insert(self, image: ImageBuffer<PB, PCB>) -> Self::Insert {
        let ContextRasterImage { context_raster, .. } = self;
        ContextRasterImage {
            image,
            context_raster,
        }
    }
}

impl<C, R, I> Remove<Context<C>> for ContextRasterImage<Context<C>, R, I> {
    type Remove = ContextRasterImage<(), R, I>;

    fn remove(self) -> (Self::Remove, Context<C>) {
        let ContextRasterImage {
            context_raster,
            image,
        } = self;
        let (context_raster, context) = context_raster.remove();
        (
            ContextRasterImage {
                context_raster,
                image,
            },
            context,
        )
    }
}

impl<C, R, I> Remove<Raster<R>> for ContextRasterImage<C, Raster<R>, I> {
    type Remove = ContextRasterImage<C, (), I>;

    fn remove(self) -> (Self::Remove, Raster<R>) {
        let ContextRasterImage {
            context_raster,
            image,
        } = self;
        let (context_raster, raster) = context_raster.remove();
        (
            ContextRasterImage {
                context_raster,
                image,
            },
            raster,
        )
    }
}

impl<C, R, P, PC> Remove<ImageBuffer<P, PC>> for ContextRasterImage<C, R, ImageBuffer<P, PC>>
where
    P: Pixel,
{
    type Remove = ContextRasterImage<C, R, ()>;

    fn remove(self) -> (Self::Remove, ImageBuffer<P, PC>) {
        let ContextRasterImage {
            context_raster,
            image,
        } = self;
        (
            ContextRasterImage {
                context_raster,
                image: (),
            },
            image,
        )
    }
}

impl<C, R, I> Empty for ContextRasterImage<C, R, I> {
    type Empty = ContextRasterImage<(), (), ()>;

    fn empty() -> Self::Empty {
        ContextRasterImage {
            context_raster: ContextRaster::<C, R>::empty(),
            image: (),
        }
    }
}

impl<C, R, I, U> UnionWith<U> for ContextRasterImage<C, R, I>
where
    U: Insert<C>,
    InsertT<U, C>: Insert<R>,
    InsertT<InsertT<U, C>, R>: Insert<I>,
{
    type UnionWith = InsertT<InsertT<InsertT<U, C>, R>, I>;

    fn union_with(self, u: U) -> Self::UnionWith {
        self.context_raster.union_with(u).insert(self.image)
    }
}

impl<C, R, I, U> SubtractFrom<U> for ContextRasterImage<C, R, I>
where
    U: Drop<C>,
    DropT<U, C>: Drop<R>,
    DropT<DropT<U, C>, R>: Drop<I>,
{
    type SubtractFrom = DropT<DropT<DropT<U, C>, R>, I>;

    fn subtract_from(self, u: U) -> Self::SubtractFrom {
        u.drop().drop().drop()
    }
}
