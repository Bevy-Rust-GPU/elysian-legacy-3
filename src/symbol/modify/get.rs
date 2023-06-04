use std::marker::PhantomData;

use t_funk::{collection::set::GetF, typeclass::functor::Fmap};

use crate::{LiftAdt, LiftModifier, Modify};

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

impl<T> LiftAdt for Get<T> {
    type LiftAdt = Modify<Self>;

    fn adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T> LiftModifier for Get<T> {
    type LiftModifier = GetF<T>;

    fn lift_modifier(self) -> Self::LiftModifier {
        GetF::<T>::default()
    }
}
