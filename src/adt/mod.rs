//! Elysian ADT
//! Elysian
//! = Input a
//! | Field b
//! | Output c
//! | Modify d
//! | Sequence [In|Field|Out|Modify]
//! | Combine Field|Shape|Combine Field|Shape|Combine f
//! where
//!   a: InputModifer
//!   b: FieldMorphism
//!   c: OutputModifier
//!   f: CombineFunction
//!
//! Example:
//!
//! Shape [
//!   In Translate -0.1 -0.3,
//!   Combine (
//!     Shape [
//!       In Translate 0.2 0.2,
//!       Field Point,
//!       Out Isosurface 0.3,
//!     ],
//!     Shape [
//!       In Translate -0.2 -0.2,
//!       Field Point,
//!       Out Isosurface 0.5,
//!       Out Manifold,
//!     ],
//!     Boolean(Lt),
//!   ),
//!   Out Isosurface 0.2,
//! ]
//!

mod algebra;
mod builder;
mod impls;

pub use algebra::*;
pub use builder::*;
pub use impls::*;

mod bounds;
pub(crate) use bounds::*;

use t_funk::macros::{Copointed, Pointed};

/// A -> A input modifier function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input<T>(pub T);

/// A -> B field function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field<T>(pub T);

/// B -> B output modifier function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Output<T>(pub T);

/// C -> C context modifier function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Modify<T>(pub T);

/// Recursive list of Input / Field / Output items
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sequence<A, B>(pub A, pub B);

/// Combine ADT items A, B using function F
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Combine<A, B, F>(pub A, pub B, pub F);

#[cfg(test)]
mod test {
    use t_funk::closure::Const;

    use crate::{
        DistGrad, Distance, Do, Done, Evaluate, Get, Isosurface, Point, PosDistGrad, Translate,
    };

    #[test]
    fn test_adt() {
        // Shape
        let shape_a =
            Do >> Translate(Const(0.5), Const(0.5)) >> Point >> Isosurface(Const(0.2)) >> Done;
        let shape_b = Do >> Point >> Isosurface(Const(0.2)) >> Done;
        let union = Do >> shape_a + shape_b >> Done;
        let combined =
            Do >> union >> Isosurface(Const(0.1)) >> Get::<Distance<f32>>::default() >> Done;

        let shape = Evaluate::<DistGrad<f32>, _>::evaluate(combined, PosDistGrad::default());

        assert_eq!(shape, Distance(-0.3));
    }
}
