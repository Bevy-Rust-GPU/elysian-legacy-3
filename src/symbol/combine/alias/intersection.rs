use t_funk::{
    function::Gt,
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
pub trait MakeIntersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

impl<T, U> MakeIntersection<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type Intersection = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<Intersection>>;

    fn intersection(self, rhs: U) -> Self::Intersection {
        Combine(
            self.into_tuple(),
            rhs.into_tuple(),
            Intersection.into_tuple(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection;

impl<F> Fmap<F> for Intersection {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for Intersection {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Intersection {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl ExpandAlias<Dist<f32>> for Intersection {
    type ExpandAlias = ExpandAliasT<EvaluateSelect<Inherited, Distance<f32>, Gt>, Dist<f32>>;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

impl<D> ExpandAlias<(Distance<f32>, D)> for Intersection
where
    D: Pair,
{
    type ExpandAlias = ExpandAliasT<
        EvaluatePredicated<Dist<f32>, Inherited, Distance<f32>, Gt>,
        (Distance<f32>, D),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
