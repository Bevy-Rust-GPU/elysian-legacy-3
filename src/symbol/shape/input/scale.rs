use core::ops::{Div, Mul};

use crate::{
    Alias, Distance, EvaluateFunction, EvaluateInputs, ExpandAlias, ExpandAliasF, IntoMonad,
    IntoTuple, IntoTupleT, LiftAdt, LiftAdtF, Modify, Position,
};

use crate::glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT, Identity},
        semigroup::{Mappend, MappendT},
    },
};

pub trait Scale<U> {
    type Scale;

    fn scale(self, s: U) -> Self::Scale;
}

impl<T, U> Scale<U> for T
where
    T: IntoTuple,
{
    type Scale = Scaler<U, IntoTupleT<T>>;

    fn scale(self, s: U) -> Self::Scale {
        Scaler(s, self.into_tuple())
    }
}

// Wrapper to pre-scale a Position, then post-inverse-scale a resulting Distance
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scaler<S, T>(pub S, pub T);

impl<S, T, F> Fmap<F> for Scaler<S, T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<S, T> IntoMonad for Scaler<S, T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<S, T> LiftAdt for Scaler<S, T>
where
    T: Fmap<LiftAdtF>,
{
    type LiftAdt = Alias<Scaler<S, FmapT<T, LiftAdtF>>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(Scaler(self.0, self.1.fmap(LiftAdtF)))
    }
}

impl<S, T, D> ExpandAlias<D> for Scaler<S, T>
where
    S: Clone,
    T: Chain<ExpandAliasF<D>>,
    (ScalePosition<S>,): Mappend<ChainT<T, ExpandAliasF<D>>>,
    MappendT<(ScalePosition<S>,), ChainT<T, ExpandAliasF<D>>>: Mappend<(InverseScaleDistance<S>,)>,
{
    type ExpandAlias = MappendT<
        MappendT<(ScalePosition<S>,), ChainT<T, ExpandAliasF<D>>>,
        (InverseScaleDistance<S>,),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        (ScalePosition(self.0.clone()),)
            .mappend(self.1.chain(ExpandAliasF::<D>::default()))
            .mappend((InverseScaleDistance(self.0),))
    }
}

// Position scaler
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalePosition<S>(pub S);

impl<S> LiftAdt for ScalePosition<S> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<S, D> EvaluateInputs<D> for ScalePosition<S> {
    type Inputs = Position<Vec2>;
    type Moves = ();
}

impl<S, D> EvaluateFunction<D> for ScalePosition<S> {
    type Function = Curry2B<ScalePositionF, S>;

    fn evaluate_function(self) -> Self::Function {
        ScalePositionF.suffix2(self.0)
    }
}

#[lift]
pub fn scale_position_f<P, S>(Position(p): Position<P>, scale: S) -> Position<P::Output>
where
    P: Div<S>,
{
    Position(p / scale)
}

// Distance inverse scaler
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InverseScaleDistance<S>(pub S);

impl<S> LiftAdt for InverseScaleDistance<S> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<S, D> EvaluateInputs<D> for InverseScaleDistance<S> {
    type Inputs = Distance<f32>;
    type Moves = ();
}

impl<S, D> EvaluateFunction<D> for InverseScaleDistance<S> {
    type Function = Curry2B<InverseScaleDistanceF, S>;

    fn evaluate_function(self) -> Self::Function {
        InverseScaleDistanceF.suffix2(self.0)
    }
}

#[lift]
pub fn inverse_scale_distance_f<D, S>(Distance(d): Distance<D>, scale: S) -> Distance<D::Output>
where
    D: Mul<S>,
{
    Distance(d * scale)
}
