use t_funk::{
    closure::{Compose, ComposeLT},
    collection::set::GetF,
    function::Gt,
    typeclass::arrow::{Split, SplitT},
};

use crate::{Bounding, Distance, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBound;

impl<D> LiftCombine<D> for OuterBound {
    type LiftCombine = Bounding<ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Gt>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Bounding(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt),
        )
    }
}
