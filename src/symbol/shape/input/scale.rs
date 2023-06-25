use std::ops::{Div, Mul};

use crate::{
    Alias, Distance, Evaluable, EvaluateFunction, ExpandAlias, ExpandAliasF, IntoMonad, LiftAdt,
    LiftAdtF, LiftModify, Position, Run,
};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT, Identity},
        semigroup::{Mappend, MappendT},
    },
};

// Wrapper to pre-scale a Position, then post-inverse-scale a resulting Distance
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale<S, T>(pub S, pub T);

impl<S, T, F> Fmap<F> for Scale<S, T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<S, T> LiftAdt for Scale<S, T>
where
    T: Fmap<LiftAdtF>,
{
    type LiftAdt = Alias<Scale<S, FmapT<T, LiftAdtF>>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(Scale(self.0, self.1.fmap(LiftAdtF)))
    }
}

impl<S, T> ExpandAlias for Scale<S, T>
where
    S: Clone,
    T: Chain<ExpandAliasF>,
    (ScalePosition<S>,): Mappend<ChainT<T, ExpandAliasF>>,
    MappendT<(ScalePosition<S>,), ChainT<T, ExpandAliasF>>: Mappend<(InverseScaleDistance<S>,)>,
{
    type ExpandAlias = MappendT<
        MappendT<(ScalePosition<S>,), ChainT<T, ExpandAliasF>>,
        (InverseScaleDistance<S>,),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        (ScalePosition(self.0.clone()),)
            .mappend(self.1.chain(ExpandAliasF))
            .mappend((InverseScaleDistance(self.0),))
    }
}

impl<S, T> IntoMonad for Scale<S, T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

// Position scaler
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalePosition<S>(pub S);

impl<S> LiftAdt for ScalePosition<S> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<S> Evaluable for ScalePosition<S> {
    type Evaluable = LiftModify;
}

impl<S, D> EvaluateFunction<D> for ScalePosition<S> {
    type Inputs = Position<Vec2>;
    type Moves = ();
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
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<S> Evaluable for InverseScaleDistance<S> {
    type Evaluable = LiftModify;
}

impl<S, D> EvaluateFunction<D> for InverseScaleDistance<S> {
    type Inputs = Distance<f32>;
    type Moves = ();
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
