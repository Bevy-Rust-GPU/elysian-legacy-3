use t_funk::{
    closure::ComposeLT,
    collection::set::GetF,
    function::Gt,
    typeclass::{
        arrow::{Split, SplitT},
        category::ComposeL,
    },
};

use crate::{Boolean, Distance, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection;

impl LiftCombine for Intersection {
    type LiftCombine = Boolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Boolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}
