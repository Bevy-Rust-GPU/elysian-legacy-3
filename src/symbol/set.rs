use t_funk::{
    closure::{Closure, OutputT},
    closure::{Curry2, Curry2B},
    collection::set::InsertF,
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    }, macros::types,
};

use crate::{
    EvaluateFunction, IntoMonad, IntoTuple, IntoTupleT, LiftAdt, LiftParam, LiftParamT, Run,
};

#[types]
pub trait Set<T> {
    type Set;

    fn set(self, t: T) -> Self::Set;
}

impl<T, U> Set<U> for T
where
    T: IntoTuple,
    SetS<U>: IntoTuple,
    IntoTupleT<T>: Mappend<IntoTupleT<SetS<U>>>,
{
    type Set = MappendT<IntoTupleT<T>, IntoTupleT<SetS<U>>>;

    fn set(self, t: U) -> Self::Set {
        self.into_tuple().mappend(SetS(t).into_tuple())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetS<T>(pub T);

impl<T, F> Fmap<F> for SetS<T>
where
    F: Closure<T>,
{
    type Fmap = SetS<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        SetS(f.call(self.0))
    }
}

impl<T> IntoMonad for SetS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for SetS<T> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, C> LiftParam<C> for SetS<T>
where
    T: LiftParam<C>,
{
    type LiftParam = SetS<LiftParamT<T, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        SetS(self.0.lift_param(input))
    }
}

impl<T, D> EvaluateFunction<D> for SetS<T> {
    type Function = Curry2B<InsertF, T>;

    fn evaluate_function(self) -> Self::Function {
        InsertF.suffix2(self.0)
    }
}
