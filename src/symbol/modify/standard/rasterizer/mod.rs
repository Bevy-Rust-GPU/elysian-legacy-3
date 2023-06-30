mod raster;
pub use raster::*;

use core::marker::PhantomData;

use crate::glam::Vec2;
use t_funk::{
    closure::Closure,
    collection::set::{Insert, InsertT},
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Context, EvaluateFunction, EvaluateImpl, EvaluateImplT, EvaluateInputs, IntoMonad, IntoTuple,
    IntoTupleT, LiftAdt, Modify, Position,
};

pub trait Rasterize {
    type Rasterize<C>;

    fn rasterize<C>(self, w: usize, h: usize) -> Self::Rasterize<C>;
}

impl<T> Rasterize for T {
    type Rasterize<C> = Rasterizer<T, C>;

    fn rasterize<C>(self, width: usize, height: usize) -> Self::Rasterize<C> {
        Rasterizer {
            width,
            height,
            shape: self,
            context: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rasterizer<S, C> {
    pub width: usize,
    pub height: usize,
    pub shape: S,
    pub context: PhantomData<C>,
}

impl<S, C> Default for Rasterizer<S, C>
where
    S: Default,
{
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            shape: Default::default(),
            context: Default::default(),
        }
    }
}

impl<S, C> Clone for Rasterizer<S, C>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: self.height.clone(),
            shape: self.shape.clone(),
            context: self.context.clone(),
        }
    }
}

impl<S, C> Copy for Rasterizer<S, C> where S: Copy {}

impl<S, C, F> Fmap<F> for Rasterizer<S, C> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<S, C> IntoMonad for Rasterizer<S, C> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<S, C> LiftAdt for Rasterizer<S, C> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<S, C, D> EvaluateInputs<D> for Rasterizer<S, C> {
    type Inputs = Context<C>;
    type Moves = Context<C>;
}

impl<S, C, D> EvaluateFunction<D> for Rasterizer<S, C> {
    type Function = RasterizeF<S, D>;

    fn evaluate_function(self) -> Self::Function {
        RasterizeF {
            width: self.width,
            height: self.height,
            shape: self.shape,
            domain: PhantomData,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterizeF<S, D> {
    width: usize,
    height: usize,
    shape: S,
    domain: PhantomData<D>,
}

impl<D, C, S> Closure<Context<C>> for RasterizeF<S, D>
where
    S: Clone + IntoTuple,
    IntoTupleT<S>: Clone + EvaluateImpl<D, InsertT<C, Position<Vec2>>>,
    EvaluateImplT<IntoTupleT<S>, D, InsertT<C, Position<Vec2>>>: Default + Clone,
    C: Clone + Insert<Position<Vec2>>,
{
    type Output = Raster<EvaluateImplT<IntoTupleT<S>, D, InsertT<C, Position<Vec2>>>>;

    fn call(self, Context(ctx): Context<C>) -> Self::Output {
        let mut out: Self::Output = Raster::new(self.width, self.height);
        for (y, row) in out.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let nx = ((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / self.height as f32) * 2.0 - 1.0;
                *col = EvaluateImpl::<D, InsertT<C, Position<Vec2>>>::evaluate_impl(
                    self.shape.clone().into_tuple(),
                    ctx.clone().insert(Position(Vec2::new(nx, ny))),
                );
            }
        }
        out
    }
}
