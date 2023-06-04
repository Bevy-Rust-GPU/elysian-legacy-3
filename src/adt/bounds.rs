//! Bounding traits for the ADT

use crate::{Combine, Field, Input, Output, Sequence, Modify};

// Trait identifying all ADT members

pub trait Elysian {}
impl<A> Elysian for Input<A> {}
impl<A> Elysian for Field<A> {}
impl<A> Elysian for Output<A> {}
impl<A> Elysian for Modify<A> {}
impl<A, B> Elysian for Sequence<A, B> {}
impl<A, B, F> Elysian for Combine<A, B, F> {}

// Traits identifying identifying a single ADT member

pub trait IsInput {}
impl<A> IsInput for Input<A> {}

pub trait IsField {}
impl<A> IsField for Field<A> {}

pub trait IsOutput {}
impl<A> IsOutput for Output<A> {}

pub trait IsModify {}
impl<A> IsModify for Modify<A> {}

pub trait IsShape {}
impl<A, B> IsShape for Sequence<A, B> {}

pub trait IsCombine {}
impl<A, B, F> IsCombine for Combine<A, B, F> {}

// Traits identifying identifying all but one ADT member
// Used to emulate negative type bounds

pub trait NotInput {}
impl<A> NotInput for Field<A> {}
impl<A> NotInput for Output<A> {}
impl<A> NotInput for Modify<A> {}
impl<A, B> NotInput for Sequence<A, B> {}
impl<A, B, F> NotInput for Combine<A, B, F> {}

pub trait NotField {}
impl<A> NotField for Input<A> {}
impl<A> NotField for Output<A> {}
impl<A> NotField for Modify<A> {}
impl<A, B> NotField for Sequence<A, B> {}
impl<A, B, F> NotField for Combine<A, B, F> {}

pub trait NotOutput {}
impl<A> NotOutput for Input<A> {}
impl<A> NotOutput for Field<A> {}
impl<A> NotOutput for Modify<A> {}
impl<A, B> NotOutput for Sequence<A, B> {}
impl<A, B, F> NotOutput for Combine<A, B, F> {}

pub trait NotModify {}
impl<A> NotModify for Input<A> {}
impl<A> NotModify for Field<A> {}
impl<A> NotModify for Output<A> {}
impl<A, B> NotModify for Sequence<A, B> {}
impl<A, B, F> NotModify for Combine<A, B, F> {}

pub trait NotSequence {}
impl<A> NotSequence for Input<A> {}
impl<A> NotSequence for Field<A> {}
impl<A> NotSequence for Output<A> {}
impl<A> NotSequence for Modify<A> {}
impl<A, B, F> NotSequence for Combine<A, B, F> {}

pub trait NotCombine {}
impl<A> NotCombine for Field<A> {}
impl<A> NotCombine for Input<A> {}
impl<A> NotCombine for Output<A> {}
impl<A> NotCombine for Modify<A> {}
impl<A, B> NotCombine for Sequence<A, B> {}
