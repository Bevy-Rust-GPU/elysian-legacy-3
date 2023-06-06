mod impls;

pub use impls::*;

use t_funk::macros::{Copointed, Pointed};

use crate::{LiftAdt, Unit};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modify<T>(pub T);

impl<T> LiftAdt for Modify<T> {
    type LiftAdt = Unit<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Unit(self)
    }
}
