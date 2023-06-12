mod raster;
pub use raster::*;

use std::marker::PhantomData;

use glam::Vec2;
use t_funk::{
    closure::Closure,
    collection::set::{Insert, InsertT},
    typeclass::functor::Fmap,
};

use crate::{Context, Evaluate, EvaluateT, LiftAdt, Modify, EvaluateFunction, Position};

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

impl<S, C> LiftAdt for Rasterizer<S, C> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<S, C, D> EvaluateFunction<D> for Rasterizer<S, C> {
    type Inputs = Context<C>;
    type Moves = ();
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
    S: Clone + Evaluate<D, InsertT<C, Position<Vec2>>>,
    EvaluateT<S, D, InsertT<C, Position<Vec2>>>: Default + Clone,
    C: Clone + Insert<Position<Vec2>>,
{
    type Output = Raster<EvaluateT<S, D, InsertT<C, Position<Vec2>>>>;

    fn call(self, Context(ctx): Context<C>) -> Self::Output {
        let mut out: Self::Output = Raster::new(self.width, self.height);
        for (y, row) in out.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                let nx = ((x as f32 + 0.5) / self.width as f32) * 2.0 - 1.0;
                let ny = ((y as f32 + 0.5) / self.height as f32) * 2.0 - 1.0;
                *col = Evaluate::<D, InsertT<C, Position<Vec2>>>::evaluate(
                    self.shape.clone(),
                    ctx.clone().insert(Position(Vec2::new(nx, ny))),
                );
            }
        }
        out
    }
}
