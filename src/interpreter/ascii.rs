extern crate alloc;

use core::marker::PhantomData;

use alloc::{format, string::String};

use crate::{Distance, DistanceF32, Domain, PositionF32, Evaluate, EvaluateT};

use type_fields::t_funk::{
    closure::Compose, function::OutputT, hlist::Nil, Closure, Composed, Fst, Function,
};

use super::Rasterize;

pub type Ramp<const N: usize> = [char; N];
pub const ASCII_RAMP: Ramp<11> = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '█'];

pub struct Ascii<const EX: usize, const EY: usize, T>(PhantomData<T>);

impl<const EX: usize, const EY: usize, T> Default for Ascii<EX, EY, T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<const EX: usize, const EY: usize, T> Clone for Ascii<EX, EY, T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<const EX: usize, const EY: usize, T> Copy for Ascii<EX, EY, T> {}

impl<const EX: usize, const EY: usize, const N: usize, T, S> Function<(Ramp<N>, S)>
    for Ascii<EX, EY, T>
where
    S: Clone,
    S: Domain<Evaluate>,
    Fst: Compose<EvaluateT<S>>,
    Composed<Fst, EvaluateT<S>>: Clone + Closure<(PositionF32, Nil), Output = DistanceF32>,
{
    type Output = String;

    fn call((ramp, shape): (Ramp<N>, S)) -> Self::Output {
        let rast = Rasterize::<EX, EY, Distance<f32>>::default().call(shape);

        let mut string = String::default();
        for y in 0..EY {
            for x in 0..EX {
                let dist_norm = 1.0 - rast[y][x].0.max(0.0).min(1.0);
                let idx = (dist_norm * (N - 1) as f32) as usize;
                let char = ramp[idx];
                string += &format!("{char:}");
            }

            string += "\n";
        }

        string
    }
}

impl<const EX: usize, const EY: usize, const N: usize, T, S> Closure<(Ramp<N>, S)>
    for Ascii<EX, EY, T>
where
    Self: Function<([char; N], S)>,
{
    type Output = OutputT<Ascii<EX, EY, T>, ([char; N], S)>;

    fn call(self, input: ([char; N], S)) -> Self::Output {
        <Self as Function<([char; N], S)>>::call(input)
    }
}
