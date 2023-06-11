use t_funk::{
    closure::{Compose, ComposeLT, Composed},
    collection::set::GetF,
    function::{Gt, Neg},
    typeclass::arrow::{First, Firsted, Split, SplitT},
};

use crate::{Distance, LiftCombine, Pair, PostBoolean};

use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{Combine, LiftAdtF, Then};

#[functions]
#[types]
pub trait Subtraction<R> {
    type Subtraction;

    fn subtraction(self, rhs: R) -> Self::Subtraction;
}

pub fn subtraction() -> OpChain<LiftAdtF, SubtractionF> {
    Default::default()
}

impl_adt! {
    impl<A, B, C, R> Subtraction<R> for Then<A, B> | Combine<A, B, C> {
        type Subtraction = Combine<Self, R, SubtractionS>;

        fn subtraction(self, rhs: R) -> Self::Subtraction {
            Combine(self, rhs, SubtractionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubtractionS;

impl LiftCombine<(Distance<f32>, ())> for SubtractionS {
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

impl<D> LiftCombine<(Distance<f32>, D)> for SubtractionS
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
