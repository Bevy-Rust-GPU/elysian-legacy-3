extern crate alloc;

use alloc::string::String;
use glam::Vec2;

use crate::{AsUsize, CharsToString, Dist, Distance, Index, Invert, PosDist, Raster, Saturate};

use t_funk::{
    closure::{Compose, Composed, Curry2, Curry2A},
    collection::set::{Get, GetF},
    function::{Mul, PrintLn},
    macros::lift,
    typeclass::{
        copointed::{CopointF, Copointed},
        functor::Fmap,
    },
};

use super::Rasterize;

pub type Ramp<const N: usize> = [char; N];
pub const ASCII_RAMP: Ramp<11> = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

#[lift]
pub fn ascii<const N: usize, C>(ramp: Ramp<N>, rast: Raster<C>) -> String
where
    C: Clone + Get<Distance<f32>>,
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

pub fn make_ascii(
    width: usize,
    height: usize,
) -> Composed<
    PrintLn,
    Composed<Curry2A<Ascii, [char; 11]>, Curry2A<Rasterize<Dist<f32>>, PosDist<Vec2, f32>>>,
> {
    Rasterize {
        width,
        height,
        ..Default::default()
    }
    .prefix2(PosDist::default())
    .compose_l(Ascii.prefix2(ASCII_RAMP))
    .compose_l(PrintLn)
}
