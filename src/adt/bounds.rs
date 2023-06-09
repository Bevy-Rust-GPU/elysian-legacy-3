//! Bounding traits for the ADT

use t_funk::macros::impl_adt;

use crate::{Combine, Run, Then};

// Traits identifying identifying all but one ADT member
// Used to emulate negative type bounds

pub trait NotAdtEnd {}

impl_adt! {
    impl<A, B, C> NotAdtEnd for Run<A> | Then<A, B> | Combine<A, B, C> {}
}
