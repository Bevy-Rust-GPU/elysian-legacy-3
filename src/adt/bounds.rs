//! Bounding traits for the ADT

use t_funk::macros::impl_adt;

use crate::{Combine, Nil, Sequence, Unit};

// Traits identifying identifying all but one ADT member
// Used to emulate negative type bounds

pub trait NotNil {}

impl_adt! {
    impl<A, B, C> NotNil for Unit<A> | Sequence<A, B> | Combine<A, B, C> {}
}

pub trait NotCombine {}

impl_adt! {
    impl<A, B> NotCombine for Nil | Unit<A> | Sequence<A, B> {}
}
