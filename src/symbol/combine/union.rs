use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Lt,
    typeclass::arrow::{Split, SplitT},
};

use crate::{PostBoolean, Distance, LiftCombine, Pair, PreBoolean};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union;

impl LiftCombine<(Distance<f32>, ())> for Union {
    type LiftCombine = PostBoolean<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Lt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        PostBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Lt),
        )
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for Union
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
