use std::marker::PhantomData;

use t_funk::{
    collection::set::Get,
    macros::phantom::{PhantomClone, PhantomCopy, PhantomDefault},
};

use crate::LiftParam;

#[derive(
    Debug, PhantomDefault, PhantomClone, PhantomCopy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParamGet<T>(pub PhantomData<T>);

impl<T, C> LiftParam<C> for ParamGet<T>
where
    C: Get<T>,
{
    type LiftParam = T;

    fn lift_param(self, input: C) -> Self::LiftParam {
        input.get()
    }
}
