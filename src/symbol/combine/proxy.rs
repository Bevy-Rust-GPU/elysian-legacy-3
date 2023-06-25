use std::marker::PhantomData;

use t_funk::{
    function::{Id, Lt},
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, Distance,
    EvaluateSide, Inherited, Left, LiftEvaluate, Right,
};

use t_funk::macros::types;

use crate::Combine;

#[types]
pub trait Proxy<U> {
    type Proxy<T>;

    fn proxy<T>(self, rhs: U) -> Self::Proxy<T>;
}

impl<T, R> Proxy<R> for T {
    type Proxy<U> = Combine<Self, R, Identity<ProxyS<U>>>;

    fn proxy<U>(self, rhs: R) -> Self::Proxy<U> {
        Combine(self, rhs, Identity(ProxyS(PhantomData::<U>)))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProxyS<T>(pub PhantomData<T>);

impl<T, D> LiftEvaluate<D> for ProxyS<T> {
    type LiftEvaluate = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        CopyContext<ContextA, ContextOut>,
        BooleanConditional<Lt, Id, CopyProperty<T, ContextB, ContextOut>, Distance<f32>>,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
