use t_funk::{
    collection::hlist::{Cons, Nil},
    function::Lt,
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance, EvaluateSide,
    Inherited, Left, LiftEvaluate, Pair, Right,
};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Run, Then};

pub fn union() -> OpChain<LiftAdtF, UnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait Union<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl_adt! {
    impl<A, B, C, R> Union<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type Union = Combine<Self, R, Identity<UnionS>>;

        fn union(self, rhs: R) -> Self::Union {
            Combine(self, rhs, Identity(UnionS))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionS;

impl LiftEvaluate<Dist<f32>> for UnionS {
    type LiftEvaluate = Cons<
        EvaluateSide<Left, Inherited, ContextA>,
        Cons<
            EvaluateSide<Right, Inherited, ContextB>,
            Cons<
                BooleanConditional<
                    Lt,
                    CopyContext<ContextA, ContextOut>,
                    CopyContext<ContextB, ContextOut>,
                    Distance<f32>,
                >,
                Nil,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}

impl<D> LiftEvaluate<(Distance<f32>, D)> for UnionS
where
    D: Pair,
{
    type LiftEvaluate = Cons<
        EvaluateSide<Left, Dist<f32>, ContextA>,
        Cons<
            EvaluateSide<Right, Dist<f32>, ContextB>,
            Cons<
                BooleanConditional<
                    Lt,
                    EvaluateSide<Left, Inherited, ContextOut>,
                    EvaluateSide<Right, Inherited, ContextOut>,
                    Distance<f32>,
                >,
                Nil,
            >,
        >,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Default::default()
    }
}
