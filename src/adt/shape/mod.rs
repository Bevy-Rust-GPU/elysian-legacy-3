mod bounds;
mod impls;

pub use bounds::*;
pub use impls::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Shape
      // A -> A
      = Input<A, B>(pub A, pub B)
      // A -> B
      | Field<A, B>(pub A, pub B)
      // B -> B
      | Output<A, B>(pub A, pub B)
      // Terminating type
      | ShapeEnd;
);
