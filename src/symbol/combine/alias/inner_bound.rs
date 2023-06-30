use core::marker::PhantomData;

use t_funk::{
    closure::{Curry2, Curry2B},
    function::Lt,
    macros::{functions, types},
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, Combine, ContextOut, Dist, Distance, EvaluateSide, ExpandAlias, Inherited,
    InsertProperty, IntoMonad, IntoMonadT, IntoTuple, IntoTupleT, Left, LiftAdt, Right,
    UnaryConditional,
};

#[functions]
#[types]
pub trait MakeInnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

impl<T, U> MakeInnerBound<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type InnerBound = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoMonadT<InnerBound>>;

    fn inner_bound(self, rhs: U) -> Self::InnerBound {
        Combine(self.into_tuple(), rhs.into_tuple(), InnerBound.into_monad())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InnerBound;

impl<F> Fmap<F> for InnerBound {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for InnerBound {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for InnerBound {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for InnerBound {
    type ExpandAlias = (
        EvaluateSide<Left, Dist<f32>, ContextOut>,
        UnaryConditional<
            ContextOut,
            Distance<f32>,
            Curry2B<Lt, Distance<f32>>,
            EvaluateSide<Right, Inherited, ContextOut>,
            InsertProperty<Distance<f32>, ContextOut>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        (
            EvaluateSide::<Left, Dist<f32>, ContextOut>::default(),
            UnaryConditional(
                Lt.suffix2(Distance(0.0)),
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                InsertProperty(Distance(f32::INFINITY), PhantomData::<ContextOut>),
                PhantomData,
            ),
        )
    }
}
