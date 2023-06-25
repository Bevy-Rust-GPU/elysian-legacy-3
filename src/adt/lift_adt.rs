//! Lift a concrete value into the Elysian ADT

use t_funk::{
    macros::{functions, impl_adt},
    op_chain::{op_chain_lift, OpChain},
    typeclass::{
        category::ComposeF,
        functor::{Fmap, FmapT},
    },
};

use crate::{Alias, Combine, Modify, Run, Domains};

#[functions]
pub trait LiftAdt {
    type LiftAdt;

    fn lift_adt(self) -> Self::LiftAdt;
}

#[allow(non_snake_case)]
pub fn adt() -> OpChain<LiftAdtF, ComposeF> {
    op_chain_lift(LiftAdtF, ComposeF)
}

pub type LiftAdtT<T> = <T as LiftAdt>::LiftAdt;

impl_adt! {
    impl<A> LiftAdt for Run<A> | Modify<A> | Domains<A> | Alias<A> {
        type LiftAdt = Self;

        fn lift_adt(self) -> Self::LiftAdt {
            self
        }
    }
}

impl<A, B, C> LiftAdt for Combine<A, B, C>
where
    A: Fmap<LiftAdtF>,
    B: Fmap<LiftAdtF>,
    C: Fmap<LiftAdtF>,
{
    type LiftAdt = Combine<FmapT<A, LiftAdtF>, FmapT<B, LiftAdtF>, FmapT<C, LiftAdtF>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Combine(
            self.0.fmap(LiftAdtF),
            self.1.fmap(LiftAdtF),
            self.2.fmap(LiftAdtF),
        )
    }
}
