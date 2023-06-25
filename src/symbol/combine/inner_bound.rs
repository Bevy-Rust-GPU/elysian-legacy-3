use std::marker::PhantomData;

use t_funk::{
    closure::{Compose, ComposeT},
    function::Lt,
    macros::{functions, types},
    typeclass::{monad::Identity, functor::Fmap},
};

use crate::{
    Alias, BooleanConditional, Combine, ContextA, ContextB, ContextOut, CopyContext, Dist,
    Distance, EvaluateSide, ExpandAlias, Inherited, InsertProperty, IntoMonad, IntoMonadT, Left,
    LiftAdt, Right,
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

impl<F> Fmap<F> for InnerBoundS {
    type Fmap = Self;

    fn fmap(self, f: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for InnerBoundS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for InnerBoundS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for InnerBoundS {
    type ExpandAlias = (
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

    fn expand_alias(self) -> Self::ExpandAlias {
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
