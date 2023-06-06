//! Bounding traits for the ADT

use crate::{Field, Input, Output, Sequence};

pub trait NotNil {}
impl<A, B> NotNil for Input<A, B> {}
impl<A, B> NotNil for Field<A, B> {}
impl<A, B> NotNil for Output<A, B> {}

// Traits identifying identifying all but one ADT member
// Used to emulate negative type bounds

pub trait NotCombine {}
impl<A, B> NotCombine for Field<A, B> {}
impl<A, B> NotCombine for Input<A, B> {}
impl<A, B> NotCombine for Output<A, B> {}
impl<A, B> NotCombine for Sequence<A, B> {}
