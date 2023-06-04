use t_funk::function::Lt;

use crate::{Bounding, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InnerBound;

impl LiftCombine for InnerBound {
    type LiftCombine = Bounding<Lt>;

    fn lift_combine(self) -> Self::LiftCombine {
        Bounding(Lt)
    }
}

