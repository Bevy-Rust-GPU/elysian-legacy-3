use std::marker::PhantomData;

use t_funk::{
    closure::{Curry2, Curry2A},
    collection::set::{Get, GetF},
    function::Mul,
    macros::lift,
    typeclass::{
        copointed::{CopointF, Copointed},
        functor::Fmap,
    },
};

use crate::{
    AsUsize, CharsToString, Distance, Index, Invert, LiftAdt, Modify, ModifyFunction, Raster,
    Saturate,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RasterToAscii<const N: usize, T>(pub Ramp<N>, pub PhantomData<T>);

impl<const N: usize, T, F> Fmap<F> for RasterToAscii<N, T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<const N: usize, T> LiftAdt for RasterToAscii<N, T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<const N: usize, T, D> ModifyFunction<D> for RasterToAscii<N, T> {
    type Inputs = Raster<T>;

    type Function = Curry2A<Ascii, Ramp<N>>;

    fn modify_function(self) -> Self::Function {
        Ascii.prefix2(self.0)
    }
}

pub type Ramp<const N: usize> = [char; N];
pub const ASCII_RAMP: Ramp<11> = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

#[lift]
pub fn ascii<const N: usize, T>(ramp: Ramp<N>, rast: Raster<T>) -> String
where
    T: Clone + Get<Distance<f32>>,
{
    rast.fmap(GetF::<Distance<f32>>::default())
        .fmap(CopointF)
        .fmap(Saturate)
        .fmap(Invert)
        .fmap(Mul.suffix2((N - 1) as f32))
        .fmap(AsUsize)
        .fmap(Index.prefix2(ramp))
        .copoint()
        .fmap(|mut line: Vec<char>| {
            line.push('\n');
            line
        })
        .fmap(CharsToString)
        .into_iter()
        .collect()
}
