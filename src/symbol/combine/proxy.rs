use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::{Id, Lt},
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, Distance,
    EvaluateSide, Inherited, Left, LiftCombine, Right,
};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Run, Then};

pub fn proxy<T>() -> OpChain<LiftAdtF, ProxyF<T>> {
    Default::default()
}

#[functions]
#[types]
pub trait Proxy<R, T> {
    type Proxy;

    fn proxy(self, rhs: R) -> Self::Proxy;
}

impl_adt! {
    impl<A, B, C, R, T> Proxy<R, T> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type Proxy = Combine<Self, R, ProxyS<T>>;

        fn proxy(self, rhs: R) -> Self::Proxy {
            Combine(self, rhs, ProxyS(PhantomData::<T>))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProxyS<T>(pub PhantomData<T>);

impl<T, D> LiftCombine<D> for ProxyS<T> {
    type LiftCombine = ComposeT<
        BooleanConditional<Lt, Id, CopyProperty<T, ContextB, ContextOut>, Distance<f32>>,
        ComposeT<
            CopyContext<ContextA, ContextOut>,
            ComposeT<
                EvaluateSide<Right, Inherited, ContextB>,
                EvaluateSide<Left, Inherited, ContextA>,
            >,
        >,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        EvaluateSide::<Left, Inherited, ContextA>::default()
            .compose_l(EvaluateSide::<Right, Inherited, ContextB>::default())
            .compose_l(CopyContext::<ContextA, ContextOut>::default())
            .compose_l(BooleanConditional(
                Lt,
                Id,
                CopyProperty::<T, ContextB, ContextOut>::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}
