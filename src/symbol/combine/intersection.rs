use t_funk::function::Gt;

use crate::{Boolean, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection;

impl LiftCombine for Intersection {
    type LiftCombine = Boolean<Gt>;

    fn lift_combine(self) -> Self::LiftCombine {
        Boolean(Gt)
    }
}

