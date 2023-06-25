use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Gt,
    macros::{functions, types},
    typeclass::{monad::Identity, functor::Fmap},
};

use crate::{
    BooleanConditional, Combine, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance,
    EvaluateSide, ExpandAlias, Inherited, InsertProperty, IntoMonad, IntoMonadT, Left, Right, LiftAdt, Alias,
};

#[functions]
#[types]
pub trait OuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

impl<T, U> OuterBound<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type OuterBound = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<OuterBoundS>>;

    fn outer_bound(self, rhs: U) -> Self::OuterBound {
        Combine(
            self.into_monad(),
            rhs.into_monad(),
            OuterBoundS.into_monad(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBoundS;

impl<F> Fmap<F> for OuterBoundS {
    type Fmap = Self;

    fn fmap(self, f: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for OuterBoundS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for OuterBoundS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for OuterBoundS {
    type ExpandAlias = (
        EvaluateSide<Left, (Distance<f32>, ()), ContextA>,
        CopyContext<ContextA, ContextB>,
        InsertProperty<Distance<f32>, ContextB>,
        BooleanConditional<
            Gt,
            EvaluateSide<Right, Inherited, ContextOut>,
            ComposeT<InsertProperty<Distance<f32>, ContextOut>, CopyContext<ContextB, ContextOut>>,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        (
            EvaluateSide::<Left, Dist<f32>, ContextA>::default(),
            CopyContext::<ContextA, ContextB>::default(),
            InsertProperty(Distance(0.0), PhantomData::<ContextB>),
            BooleanConditional(
                Gt,
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
