use t_funk::{
    closure::{Compose, Composed},
    function::{Gt, Neg},
    typeclass::arrow::{First, Firsted},
};

use crate::{Boolean, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Subtraction;

impl LiftCombine for Subtraction {
    type LiftCombine = Boolean<Composed<Gt, Firsted<Neg>>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Boolean(Neg.first().compose_l(Gt))
    }
}
