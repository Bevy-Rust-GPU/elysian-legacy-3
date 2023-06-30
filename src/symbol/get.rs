use core::marker::PhantomData;

use t_funk::{
    collection::set::GetF,
    macros::phantom::{PhantomClone, PhantomCopy, PhantomDefault},
    typeclass::functor::Fmap,
};

use crate::{EvaluateFunction, LiftAdt, LiftParam, Run};

#[derive(
    Debug, PhantomDefault, PhantomClone, PhantomCopy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Get<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for Get<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> LiftAdt for Get<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, D> EvaluateFunction<D> for Get<T> {
    type Function = GetF<T>;

    fn evaluate_function(self) -> Self::Function {
        GetF::<T>::default()
    }
}

impl<T, C> LiftParam<C> for Get<T> {
    type LiftParam = Self;

    fn lift_param(self, _: C) -> Self::LiftParam {
        self
    }
}
