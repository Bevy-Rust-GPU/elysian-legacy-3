use t_funk::{
    function::Lt,
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, BooleanConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance,
    EvaluateSide, ExpandAlias, Inherited, IntoMonad, IntoMonadT, Left, LiftAdt, Pair, Right,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait Union<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl<T, U> Union<U> for T
where
    T: IntoMonad,
    U: IntoMonad,
{
    type Union = Combine<IntoMonadT<T>, IntoMonadT<U>, IntoMonadT<UnionS>>;

    fn union(self, rhs: U) -> Self::Union {
        Combine(self.into_monad(), rhs.into_monad(), UnionS.into_monad())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionS;

impl<F> Fmap<F> for UnionS {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for UnionS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for UnionS {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl ExpandAlias<Dist<f32>> for UnionS {
    type ExpandAlias = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        BooleanConditional<
            Lt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

impl<D> ExpandAlias<(Distance<f32>, D)> for UnionS
where
    D: Pair,
{
    type ExpandAlias = (
        EvaluateSide<Left, Dist<f32>, ContextA>,
        EvaluateSide<Right, Dist<f32>, ContextB>,
        BooleanConditional<
            Lt,
            EvaluateSide<Left, Inherited, ContextOut>,
            EvaluateSide<Right, Inherited, ContextOut>,
            Distance<f32>,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
