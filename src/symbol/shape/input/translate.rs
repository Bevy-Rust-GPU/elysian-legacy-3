use core::ops::Sub;

use crate::{
    EvaluateFunction, EvaluateInputs, IntoMonad, IntoTuple, IntoTupleT, LiftAdt, Modify, Position,
};

use crate::glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
    typeclass::{
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

pub trait Translate<T> {
    type Translate;

    fn translate(self, t: T) -> Self::Translate;
}

impl<T, U> Translate<U> for T
where
    T: IntoTuple,
    TranslateS<U>: IntoTuple,
    IntoTupleT<TranslateS<U>>: Mappend<IntoTupleT<T>>,
{
    type Translate = MappendT<IntoTupleT<TranslateS<U>>, IntoTupleT<T>>;

    fn translate(self, t: U) -> Self::Translate {
        TranslateS(t).into_tuple().mappend(self.into_tuple())
    }
}

// Translation input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TranslateS<T>(pub T);

impl<T> IntoMonad for TranslateS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for TranslateS<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for TranslateS<T> {
    type Inputs = Position<Vec2>;
    type Moves = ();
}

impl<T, D> EvaluateFunction<D> for TranslateS<T> {
    type Function = Curry2B<TranslateF, T>;

    fn evaluate_function(self) -> Self::Function {
        TranslateF.suffix2(self.0)
    }
}

#[lift]
pub fn translate_f<P>(Position(p): Position<P>, translation: P) -> Position<P::Output>
where
    P: Sub<P>,
{
    Position(p - translation)
}
