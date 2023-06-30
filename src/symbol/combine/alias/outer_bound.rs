use core::marker::PhantomData;

use t_funk::{
    closure::{Curry2, Curry2B},
    function::Gt,
    macros::{functions, types},
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, Combine, ContextOut, Dist, Distance, EvaluateSide, ExpandAlias, Inherited,
    InsertProperty, IntoMonad, IntoTuple, IntoTupleT, Left, LiftAdt, Right, UnaryConditional,
};

#[functions]
#[types]
pub trait MakeOuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

impl<T, U> MakeOuterBound<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type OuterBound = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<OuterBound>>;

    fn outer_bound(self, rhs: U) -> Self::OuterBound {
        Combine(self.into_tuple(), rhs.into_tuple(), OuterBound.into_tuple())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBound;

impl<F> Fmap<F> for OuterBound {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for OuterBound {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for OuterBound {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for OuterBound {
    type ExpandAlias = (
        EvaluateSide<Left, Dist<f32>, ContextOut>,
        UnaryConditional<
            ContextOut,
            Distance<f32>,
            Curry2B<Gt, Distance<f32>>,
            EvaluateSide<Right, Inherited, ContextOut>,
            InsertProperty<Distance<f32>, ContextOut>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        (
            EvaluateSide::<Left, Dist<f32>, ContextOut>::default(),
            UnaryConditional(
                Gt.suffix2(Distance(0.0)),
                EvaluateSide::<Right, Inherited, ContextOut>::default(),
                InsertProperty(Distance(f32::INFINITY), PhantomData::<ContextOut>),
                PhantomData,
            ),
        )
    }
}
