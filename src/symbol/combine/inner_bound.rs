use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Lt,
    macros::{functions, types},
    typeclass::arrow::{Split, SplitT},
};

use crate::{Bounding, Distance, LiftCombine};

use t_funk::{macros::impl_adt, op_chain::OpChain};

use crate::{Combine, LiftAdtF, Run, Then};

#[functions]
#[types]
pub trait InnerBound<R> {
    type InnerBound;

    fn inner_bound(self, rhs: R) -> Self::InnerBound;
}

pub fn inner_bound() -> OpChain<LiftAdtF, InnerBoundF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> InnerBound<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type InnerBound = Combine<Self, R, InnerBoundS>;

        fn inner_bound(self, rhs: R) -> Self::InnerBound {
            Combine(self, rhs, InnerBoundS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InnerBoundS;

impl<D> LiftCombine<D> for InnerBoundS {
    type LiftCombine = Bounding<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Lt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Bounding(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Lt),
        )
    }
}
