use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Lt,
    macros::{functions, types},
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, InsertProperty, Left, LiftEvaluate, Right,
};

use t_funk::{macros::impl_adt, op_chain::OpChain};

use crate::{Combine, LiftAdtF, Run, Then};

#[functions]
#[types]
pub trait InnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

pub fn inner_bound() -> OpChain<LiftAdtF, InnerBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> InnerBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type InnerBound = Combine<Self, R, Identity<InnerBoundS>>;

        fn inner_bound(self, rhs: R) -> Self::InnerBound {
            Combine(self, rhs, Identity(InnerBoundS))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InnerBoundS;

impl<D> LiftEvaluate<D> for InnerBoundS {
    type LiftEvaluate = (
        EvaluateSide<Left, (Distance<f32>, ()), ContextA>,
        CopyContext<ContextA, ContextB>,
        InsertProperty<Distance<f32>, ContextB>,
        BooleanConditional<
            Lt,
            EvaluateSide<Right, Inherited, ContextOut>,
            ComposeT<InsertProperty<Distance<f32>, ContextOut>, CopyContext<ContextB, ContextOut>>,
            Distance<f32>,
        >,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        (
            EvaluateSide::<Left, Dist<f32>, ContextA>::default(),
            CopyContext::<ContextA, ContextB>::default(),
            InsertProperty(Distance(0.0), PhantomData::<ContextB>),
            BooleanConditional(
                Lt,
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                CopyContext::<ContextB, ContextOut>::default().compose_l(InsertProperty(
                    Distance(f32::INFINITY),
                    PhantomData::<ContextOut>,
                )),
                PhantomData::<Distance<f32>>,
            ),
        )
    }
}
