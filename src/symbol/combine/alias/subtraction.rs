use t_funk::{
    closure::ComposeLT,
    function::{Gt, Neg},
    typeclass::{functor::Fmap, monad::Identity, semigroup::MappendT},
};

use crate::{
    Alias, BinaryConditional, ContextA, ContextB, ContextOut, CopyContext, Dist, Distance,
    EvaluateBoth, EvaluateSide, ExpandAlias, ExpandAliasT, Inherited, IntoMonad, IntoTuple,
    IntoTupleT, Left, LiftAdt, MapProperty, Pair, Right,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait MakeSubtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

impl<T, U> MakeSubtraction<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type Subtraction = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<Subtraction>>;

    fn subtraction(self, rhs: U) -> Self::Subtraction {
        Combine(
            self.into_tuple(),
            rhs.into_tuple(),
            Subtraction.into_tuple(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Subtraction;

impl<F> Fmap<F> for Subtraction {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for Subtraction {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Subtraction {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl ExpandAlias<Dist<f32>> for Subtraction {
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<Inherited>, Dist<f32>>,
        (
            MapProperty<ContextB, Distance<f32>, Neg>,
            BinaryConditional<
                Distance<f32>,
                Gt,
                CopyContext<ContextA, ContextOut>,
                CopyContext<ContextB, ContextOut>,
            >,
        ),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

impl<D> ExpandAlias<(Distance<f32>, D)> for Subtraction
where
    D: Pair,
{
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<Dist<f32>>, (Distance<f32>, D)>,
        (
            MapProperty<ContextB, Distance<f32>, Neg>,
            BinaryConditional<
                Distance<f32>,
                Gt,
                EvaluateSide<Left, Inherited, ContextOut>,
                ComposeLT<
                    EvaluateSide<Right, Inherited, ContextOut>,
                    MapProperty<ContextOut, Distance<f32>, Neg>,
                >,
            >,
        ),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
