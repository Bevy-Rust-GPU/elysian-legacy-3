use std::marker::PhantomData;

use t_funk::{
    collection::hlist::{Cons, Nil},
    function::{Id, Lt},
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, CopyProperty, Distance,
    EvaluateSide, Inherited, Left, LiftEvaluate, Right,
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
        type Proxy = Combine<Self, R, Identity<ProxyS<T>>>;

        fn proxy(self, rhs: R) -> Self::Proxy {
            Combine(self, rhs, Identity(ProxyS(PhantomData::<T>)))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProxyS<T>(pub PhantomData<T>);

impl<T, D> LiftEvaluate<D> for ProxyS<T> {
    type LiftEvaluate = Cons<
        EvaluateSide<Left, Inherited, ContextA>,
        Cons<
            EvaluateSide<Right, Inherited, ContextB>,
            Cons<
                CopyContext<ContextA, ContextOut>,
                Cons<
                    BooleanConditional<
                        Lt,
                        Id,
                        CopyProperty<T, ContextB, ContextOut>,
                        Distance<f32>,
                    >,
                    Nil,
                >,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
