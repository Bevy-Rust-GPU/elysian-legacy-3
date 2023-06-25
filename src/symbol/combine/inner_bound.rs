use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Lt,
    macros::{functions, types},
    typeclass::monad::Identity,
};

use crate::{
    BooleanConditional, Combine, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance,
    EvaluateSide, Inherited, InsertProperty, IntoMonad, IntoMonadT, Left, LiftEvaluate, Right,
};

#[functions]
#[types]
pub trait InnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

impl<T, U> InnerBound<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type InnerBound = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<InnerBoundS>>;

    fn inner_bound(self, rhs: U) -> Self::InnerBound {
        Combine(
            self.into_monad(),
            rhs.into_monad(),
            InnerBoundS.into_monad(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InnerBoundS;

impl IntoMonad for InnerBoundS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

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
