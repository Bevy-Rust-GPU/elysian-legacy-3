use t_funk::function::Gt;

use crate::{Bounding, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OuterBound;

impl LiftCombine for OuterBound {
    type LiftCombine = Bounding<Gt>;

    fn lift_combine(self) -> Self::LiftCombine {
        Bounding(Gt)
    }
}

