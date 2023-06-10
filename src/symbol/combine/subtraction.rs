use t_funk::{
    closure::{Compose, ComposeLT, Composed},
    collection::set::GetF,
    function::{Gt, Neg},
    typeclass::arrow::{First, Firsted, Split, SplitT},
};

use crate::{Distance, LiftCombine, Pair, PostBoolean};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Subtraction;

impl LiftCombine<(Distance<f32>, ())> for Subtraction {
    type LiftCombine = PostBoolean<
        ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Composed<Gt, Firsted<Neg>>>,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        PostBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt.compose(Neg.first())),
        )
    }
}

impl<D> LiftCombine<(Distance<f32>, D)> for Subtraction
where
    D: Pair,
{
    type LiftCombine = PostBoolean<
        ComposeLT<SplitT<GetF<Distance<f32>>, GetF<Distance<f32>>>, Composed<Gt, Firsted<Neg>>>,
    >;

    fn lift_combine(self) -> Self::LiftCombine {
        PostBoolean(
            GetF::<Distance<f32>>::default()
                .split(GetF::<Distance<f32>>::default())
                .compose_l(Gt.compose(Neg.first())),
        )
    }
}
