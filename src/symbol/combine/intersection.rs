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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection;

impl LiftCombine<(Distance<f32>, ())> for Intersection {
    type LiftCombine = PostBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        PostBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for Intersection
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
