mod alias;
mod blend;
mod conditional;
mod context;
mod evaluation;
mod insert;
mod lerp;
mod monoid;
mod overwrite;

pub use alias::*;
pub use blend::*;
pub use conditional::*;
pub use context::*;
pub use evaluation::*;
pub use insert::*;
pub use lerp::*;
pub use lerp::*;
pub use monoid::*;
pub use overwrite::*;

use crate::{Combine, IntoTuple, IntoTupleT};

pub trait CombineTrait<F, T> {
    type CombineTrait;

    fn combine(self, f: F, t: T) -> Self::CombineTrait;
}

impl<T, F, U> CombineTrait<F, U> for T
where
    T: IntoTuple,
    U: IntoTuple,
    F: IntoTuple,
{
    type CombineTrait = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<F>>;

    fn combine(self, f: F, t: U) -> Self::CombineTrait {
        Combine(self.into_tuple(), t.into_tuple(), f.into_tuple())
    }
}
