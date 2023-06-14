use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Lt,
    typeclass::arrow::{Split, SplitT},
};

use crate::{
    BooleanCombine, Distance, EvaluateAndCombine, LiftCombine, Pair, PreBoolean,
};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Run, Then};

pub fn union() -> OpChain<LiftAdtF, UnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait Union<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl_adt! {
    impl<A, B, C, R> Union<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type Union = Combine<Self, R, UnionS>;

        fn union(self, rhs: R) -> Self::Union {
            Combine(self, rhs, UnionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionS;

impl LiftCombine<(Distance<f32>, ())> for UnionS {
    type LiftCombine = EvaluateAndCombine<BooleanCombine<Lt, Distance<f32>>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Default::default()
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for UnionS
where
    D: Pair,
{
    type LiftCombine = PreBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Lt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        PreBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Lt),
        )
    }
}
