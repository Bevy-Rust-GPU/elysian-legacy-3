use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Lt,
};

use crate::{
    BooleanConditional, ContextA, ContextB, ContextOut, Dist, Distance, EvaluateSide, Inherited,
    Left, LiftCombine, CopyContext, Pair, Right,
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
        type Union = Combine<Self, R, UnionS>;

        fn union(self, rhs: R) -> Self::Union {
            Combine(self, rhs, UnionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionS;

impl LiftCombine<Dist<f32>> for UnionS {
    type LiftCombine = ComposeT<
        BooleanConditional<
            Lt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
        ComposeT<EvaluateSide<Right, Inherited, ContextB>, EvaluateSide<Left, Inherited, ContextA>>,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        EvaluateSide::<Left, Inherited, ContextA>::default()
            .compose_l(EvaluateSide::<Right, Inherited, ContextB>::default())
            .compose_l(BooleanConditional(
                Lt,
                CopyContext::default(),
                CopyContext::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for UnionS
where
    D: Pair,
{
    type LiftCombine = ComposeT<
        BooleanConditional<
            Lt,
            EvaluateSide<Left, Inherited, ContextOut>,
            EvaluateSide<Right, Inherited, ContextOut>,
            Distance<f32>,
        >,
        ComposeT<EvaluateSide<Right, Dist<f32>, ContextB>, EvaluateSide<Left, Dist<f32>, ContextA>>,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        EvaluateSide::<Left, Dist<f32>, ContextA>::default()
            .compose_l(EvaluateSide::<Right, Dist<f32>, ContextB>::default())
            .compose_l(BooleanConditional(
                Lt,
                EvaluateSide::<Left, Inherited, ContextOut>::default(),
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                PhantomData::<Distance<f32>>,
            ))
    }
}
