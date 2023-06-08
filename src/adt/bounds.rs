//! Bounding traits for the ADT

use t_funk::macros::impl_adt;

use crate::{Combine, Field, Input, Modify, Output, Then};

// Traits identifying identifying all but one ADT member
// Used to emulate negative type bounds

pub trait NotEnd {}

impl_adt! {
    impl<A, B, C> NotEnd for Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C> {}
}
