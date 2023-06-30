use std::{marker::PhantomData, path::PathBuf};

use image::DynamicImage;
use t_funk::{closure::Closure, typeclass::functor::Fmap};

use crate::{EvaluateFunction, EvaluateInputs, LiftAdt, Modify};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageWriter<T>(pub PathBuf, pub PhantomData<T>);

impl<T, F> Fmap<F> for ImageWriter<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> LiftAdt for ImageWriter<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for ImageWriter<T> {
    type Inputs = T;
    type Moves = T;
}

impl<T, D> EvaluateFunction<D> for ImageWriter<T> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<T> Closure<T> for ImageWriter<T>
where
    T: Into<DynamicImage>,
{
    type Output = ();

    fn call(self, input: T) -> Self::Output {
        let input: DynamicImage = input.into().to_rgba8().into();
        input.save(self.0).expect("Failed to write image");
    }
}
