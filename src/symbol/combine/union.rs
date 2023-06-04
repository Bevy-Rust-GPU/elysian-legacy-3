use t_funk::function::Lt;

use crate::{Boolean, LiftCombine};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Union;

impl LiftCombine for Union {
    type LiftCombine = Boolean<Lt>;

    fn lift_combine(self) -> Self::LiftCombine {
        Boolean(Lt)
    }
}
