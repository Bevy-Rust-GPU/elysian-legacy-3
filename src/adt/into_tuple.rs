use t_funk::{
    macros::{lift, types, functions},
    typeclass::monad::{Chain, ChainT},
};

use crate::{IntoMonad, IntoMonadT};

#[lift]
pub fn make_tuple<T>(t: T) -> (T,) {
    (t,)
}

#[functions]
#[types]
pub trait IntoTuple {
    type IntoTuple;

    fn into_tuple(self) -> Self::IntoTuple;
}

impl<T> IntoTuple for T
where
    T: IntoMonad,
    IntoMonadT<T>: Chain<MakeTuple>,
{
    type IntoTuple = ChainT<IntoMonadT<T>, MakeTuple>;

    fn into_tuple(self) -> Self::IntoTuple {
        self.into_monad().chain(MakeTuple)
    }
}

