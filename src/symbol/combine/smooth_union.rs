use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Lt,
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
    typeclass::arrow::{Split, SplitT},
};

use crate::{Combine, Distance, LiftAdtF, LiftCombine, Run, SmoothBoolean, Then};

pub fn smooth_union() -> OpChain<LiftAdtF, SmoothUnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait SmoothUnion<T> {
    type SmoothUnion;

    fn smooth_union(self, rhs: T) -> Self::SmoothUnion;
}

impl_adt! {
    impl<A, B, C, R> SmoothUnion<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type SmoothUnion = Combine<Self, R, SmoothUnionS>;

        fn smooth_union(self, rhs: R) -> Self::SmoothUnion {
            Combine(self, rhs, SmoothUnionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothUnionS;

impl<D> LiftCombine<D> for SmoothUnionS {
    type LiftCombine =
        SmoothBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Lt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        SmoothBoolean {
            boolean: GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Lt),
            k: 0.35,
        }
    }
}
