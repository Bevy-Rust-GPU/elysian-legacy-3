use std::marker::PhantomData;

use t_funk::{
    collection::set::GetF,
    macros::phantom::{PhantomClone, PhantomCopy, PhantomDefault},
    typeclass::functor::Fmap,
};

use crate::{LiftAdt, LiftEvaluate, Run, LiftParam};

#[derive(
    Debug, PhantomDefault, PhantomClone, PhantomCopy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ContextGet<T>(pub PhantomData<T>);

impl<T, F> Fmap<F> for ContextGet<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> LiftAdt for ContextGet<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, D> LiftEvaluate<D> for ContextGet<T> {
    type LiftEvaluate = GetF<T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        GetF::<T>::default()
    }
}

impl<T, C> LiftParam<C> for ContextGet<T>
{
    type LiftParam = Self;

    fn lift_param(self, _: C) -> Self::LiftParam {
        self
    }
}
