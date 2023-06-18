use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    collection::hlist::{Cons, Nil},
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
    type LiftEvaluate = Cons<
        EvaluateSide<Left, (Distance<f32>, ()), ContextA>,
        Cons<
            CopyContext<ContextA, ContextB>,
            Cons<
                InsertProperty<Distance<f32>, ContextB>,
                Cons<
                    BooleanConditional<
                        Lt,
                        EvaluateSide<Right, Inherited, ContextOut>,
                        ComposeT<
                            InsertProperty<Distance<f32>, ContextOut>,
                            CopyContext<ContextB, ContextOut>,
                        >,
                        Distance<f32>,
                    >,
                    Nil,
                >,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Cons(
            EvaluateSide::<Left, Dist<f32>, ContextA>::default(),
            Cons(
                CopyContext::<ContextA, ContextB>::default(),
                Cons(
                    InsertProperty(Distance(0.0), PhantomData::<ContextB>),
                    Cons(
                        BooleanConditional(
                            Lt,
                            EvaluateSide::<Right, Inherited, ContextOut>::default(),
                            CopyContext::<ContextB, ContextOut>::default().compose_l(
                                InsertProperty(Distance(f32::INFINITY), PhantomData::<ContextOut>),
                            ),
                            PhantomData::<Distance<f32>>,
                        ),
                        Nil,
                    ),
                ),
            ),
        )
    }
}
