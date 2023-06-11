use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Gt,
    typeclass::arrow::{Split, SplitT},
};

use crate::{Bounding, Distance, LiftCombine};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Run, Then};

#[functions]
#[types]
pub trait OuterBound<R> {
    type OuterBound;

    fn outer_bound(self, rhs: R) -> Self::OuterBound;
}

pub fn outer_bound() -> OpChain<LiftAdtF, OuterBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> OuterBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type OuterBound = Combine<Self, R, OuterBoundS>;

        fn outer_bound(self, rhs: R) -> Self::OuterBound {
            Combine(self, rhs, OuterBoundS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBoundS;

impl<D> LiftCombine<D> for OuterBoundS {
    type LiftCombine = Bounding<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Bounding(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}
