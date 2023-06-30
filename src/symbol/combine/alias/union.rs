use t_funk::{
    function::Lt,
    typeclass::{functor::Fmap, monad::Identity},
};

use crate::{
    Alias, Dist, Distance, EvaluatePredicated, EvaluateSelect, ExpandAlias, ExpandAliasT,
    Inherited, IntoMonad, IntoTuple, IntoTupleT, LiftAdt, Pair,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait UnionTrait<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl<T, U> UnionTrait<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type Union = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<Union>>;

    fn union(self, rhs: U) -> Self::Union {
        Combine(self.into_tuple(), rhs.into_tuple(), Union.into_tuple())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union;

pub fn union() -> Union {
    Union
}

impl<F> Fmap<F> for Union {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for Union {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Union {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl ExpandAlias<Dist<f32>> for Union {
    type ExpandAlias = ExpandAliasT<EvaluateSelect<Inherited, Distance<f32>, Lt>, Dist<f32>>;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

impl<D> ExpandAlias<(Distance<f32>, D)> for Union
where
    D: Pair,
{
    type ExpandAlias = ExpandAliasT<
        EvaluatePredicated<Dist<f32>, Inherited, Distance<f32>, Lt>,
        (Distance<f32>, D),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
