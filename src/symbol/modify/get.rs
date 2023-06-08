use std::marker::PhantomData;

use t_funk::{
    collection::set::GetF,
    macros::phantom::{PhantomClone, PhantomCopy, PhantomDefault},
    typeclass::functor::Fmap,
};

use crate::{LiftAdt, LiftEvaluate, Modify};

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
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> LiftEvaluate<D> for Get<T> {
    type LiftEvaluate = GetF<T>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        GetF::<T>::default()
    }
}
