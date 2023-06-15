use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Gt,
    macros::{functions, types},
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, InsertProperty, Left, LiftCombine, Right,
};

use t_funk::{macros::impl_adt, op_chain::OpChain};

use crate::{Combine, LiftAdtF, Run, Then};

#[functions]
#[types]
pub trait OuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

pub fn outer_bound() -> OpChain<LiftAdtF, OuterBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> OuterBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type OuterBound = Combine<Self, R, OuterBoundS>;

        fn outer_bound(self, rhs: R) -> Self::OuterBound {
            Combine(self, rhs, OuterBoundS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBoundS;

impl<D> LiftCombine<D> for OuterBoundS {
    type LiftCombine = ComposeT<
        BooleanConditional<
            Gt,
            EvaluateSide<Right, Inherited, ContextOut>,
            ComposeT<InsertProperty<Distance<f32>, ContextOut>, CopyContext<ContextB, ContextOut>>,
            Distance<f32>,
        >,
        ComposeT<
            InsertProperty<Distance<f32>, ContextB>,
            ComposeT<CopyContext<ContextA, ContextB>, EvaluateSide<Left, Dist<f32>, ContextA>>,
        >,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        EvaluateSide::<Left, Dist<f32>, ContextA>::default()
            .compose_l(CopyContext::<ContextA, ContextB>::default())
            .compose_l(InsertProperty(Distance(0.0), PhantomData::<ContextB>))
            .compose_l(BooleanConditional(
                Gt,
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                CopyContext::<ContextB, ContextOut>::default().compose_l(InsertProperty(
                    Distance(f32::INFINITY),
                    PhantomData::<ContextOut>,
                )),
                PhantomData::<Distance<f32>>,
            ))
    }
}
