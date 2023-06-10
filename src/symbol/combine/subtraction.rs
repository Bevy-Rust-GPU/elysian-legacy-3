use t_funk::{
    closure::{Compose, ComposeLT, Composed},
    collection::set::GetF,
    function::{Gt, Neg},
    typeclass::arrow::{First, Firsted, Split, SplitT},
};

use crate::{Boolean, Distance, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Subtraction;

impl LiftCombine for Subtraction {
    type LiftCombine = Boolean<
        ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Composed<Gt, Firsted<Neg>>>,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        Boolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt.compose(Neg.first())),
        )
    }
}
