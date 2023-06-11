use t_funk::{
    closure::ComposeLT,
    collection::set::GetF,
    function::Gt,
    typeclass::{
        arrow::{Split, SplitT},
        category::ComposeL,
    },
};

use crate::{Distance, LiftCombine, Pair, PostBoolean, PreBoolean};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Intersection<R> {
    type Intersection;

    fn intersection(self, rhs: R) -> Self::Intersection;
}

pub fn intersection() -> OpChain<LiftAdtF, IntersectionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Intersection<R> for Then<A, B> | Combine<A, B, C> {
        type Intersection = Combine<Self, R, IntersectionS>;

        fn intersection(self, rhs: R) -> Self::Intersection {
            Combine(self, rhs, IntersectionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntersectionS;

impl LiftCombine<(Distance<f32>, ())> for IntersectionS {
    type LiftCombine = PostBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        PostBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for IntersectionS
where
    D: Pair,
{
    type LiftCombine = PreBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        PreBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}
