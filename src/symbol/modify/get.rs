use std::marker::PhantomData;

use t_funk::{collection::set::GetF, typeclass::functor::Fmap};

use crate::{LiftEvaluate, LiftModify, Modify};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Get<T>(pub PhantomData<T>);

impl<T> Default for Get<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for Get<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for Get<T> {}

impl<T, F> Fmap<F> for Get<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> LiftModify for Get<T> {
    type LiftModify = Modify<Self>;

    fn lift_modify(self) -> Self::LiftModify {
        Modify(self)
    }
}

impl<T, D> LiftEvaluate<D> for Get<T> {
    type LiftEvaluate = GetF<T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        GetF::<T>::default()
    }
}
