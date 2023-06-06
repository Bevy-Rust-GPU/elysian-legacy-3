mod impls;

pub use impls::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Input<A, B>(pub A, pub B)
             | Field<A, B>(pub A, pub B)
             | Output<A, B>(pub A, pub B)
             | Nil;
);
