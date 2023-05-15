//! Combine two shapes using a boolean operation

mod intersection;
mod subtraction;
mod union;

pub use intersection::*;
pub use subtraction::*;
pub use union::*;

use std::{fmt::Debug, marker::PhantomData};

use crate::{Domain, Evaluate, EvaluateT};

use type_fields::t_funk::{
    category::ComposeLT,
    function::Id,
    hlist::{MakeCons, Nil},
    CallF, ComposeL, Composed, Curry2, Curry2B, EitherUnwrap, Fanout, FanoutT, Fanouted, MakeIf,
    Split, Splitted, Transpose,
};

/// Combine shapes T, U using the boolean function O
pub struct Boolean<T, U, O>(pub T, pub U, pub PhantomData<O>);

impl<T, U, O> Debug for Boolean<T, U, O>
where
    T: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Conditional")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<T, U, O> Default for Boolean<T, U, O>
where
    T: Default,
    U: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), PhantomData)
    }
}

impl<T, U, O> Clone for Boolean<T, U, O>
where
    T: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<T, U, O> Copy for Boolean<T, U, O>
where
    T: Copy,
    U: Copy,
{
}

impl<T, U, O> Domain<Evaluate> for Boolean<T, U, O>
where
    T: Clone + Domain<Evaluate>,
    U: Clone + Domain<Evaluate>,
    O: Default,
    EvaluateT<T>: Fanout<EvaluateT<U>>,
    FanoutT<EvaluateT<T>, EvaluateT<U>>: ComposeL<Transpose>,
{
    type Domain = Composed<
        Splitted<Composed<EitherUnwrap, CallF>, Composed<Curry2B<MakeCons, Nil>, CallF>>,
        Composed<
            Transpose,
            Composed<
                Fanouted<Splitted<MakeIf, MakeIf>, Composed<Fanouted<Id, Id>, O>>,
                ComposeLT<FanoutT<EvaluateT<T>, EvaluateT<U>>, Transpose>,
            >,
        >,
    >;

    fn domain(self) -> Self::Domain {
        let d0 = self.0.clone().domain();
        let d1 = self.1.clone().domain();

        d0.fanout(d1)
            .compose_l(Transpose)
            .compose_l(
                MakeIf
                    .split(MakeIf)
                    .fanout(O::default().compose_l(Id.fanout(Id))),
            )
            .compose_l(Transpose)
            .compose_l(
                (CallF.compose_l(EitherUnwrap)).split(CallF.compose_l(MakeCons.suffix2(Nil))),
            )
    }
}
