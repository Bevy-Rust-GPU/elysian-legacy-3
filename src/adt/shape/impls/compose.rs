//! Composition of Shape subtypes
//!
//! ShapeEnd acts as identity for composition
//!
//! ```
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn shape_end_identity() {
//!     assert_eq!(Input((), ShapeEnd).compose(ShapeEnd), Input((), ShapeEnd));
//!     assert_eq!(Field((), ShapeEnd).compose(ShapeEnd), Field((), ShapeEnd));
//!     assert_eq!(Output((), ShapeEnd).compose(ShapeEnd), Output((), ShapeEnd));
//!
//!     assert_eq!(ShapeEnd.compose(Input((), ShapeEnd)), Input((), ShapeEnd));
//!     assert_eq!(ShapeEnd.compose(Field((), ShapeEnd)), Field((), ShapeEnd));
//!     assert_eq!(ShapeEnd.compose(Output((), ShapeEnd)), Output((), ShapeEnd));
//!
//!     assert_eq!(ShapeEnd.compose(ShapeEnd), ShapeEnd);
//! }
//! ```
//!
//! Input can compose Input and Field:
//!
//! ```
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn input_compose() {
//!     assert_eq!(Input((), ShapeEnd).compose(Input((), ShapeEnd)), Input((), Input((), ShapeEnd)));
//!     assert_eq!(Input((), ShapeEnd).compose(Field((), ShapeEnd)), Input((), Field((), ShapeEnd)));
//! }
//! ```
//!
//! Other compositions fail to compile:
//!
//! ``` compile_fail,E0277
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn input_compose_output() {
//!     Input((), ShapeEnd).compose(Output((), ShapeEnd))
//! }
//! ```
//!
//! Field can compose Output:
//!
//! ```
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn input_compose() {
//!     assert_eq!(Field((), ShapeEnd).compose(Output((), ShapeEnd)), Field((), Output((), ShapeEnd)));
//! }
//! ```
//!
//! Other compositions fail to compile:
//!
//! ``` compile_fail,E0277
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn field_compose_input() {
//!     Field((), ShapeEnd).compose(Input((), ShapeEnd))
//! }
//! ```
//!
//! ``` compile_fail,E0277
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn field_compose_field() {
//!     Field((), ShapeEnd).compose(Field((), ShapeEnd))
//! }
//! ```
//!
//! Output can compose Output:
//!
//! ```
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn output_compose() {
//!     assert_eq!(Output((), ShapeEnd).compose(Output((), ShapeEnd)), Output((), Output((), ShapeEnd)));
//! }
//! ```
//!
//! Other compositions fail to compile:
//!
//! ``` compile_fail,E0277
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn output_compose_input() {
//!     Output((), ShapeEnd).compose(Input((), ShapeEnd))
//! }
//! ```
//!
//! ``` compile_fail,E0277
//! use elysian::{Input, Field, Output, ShapeEnd};
//! use t_funk::typeclass::category::Compose;
//!
//! fn output_compose_field() {
//!     Output((), ShapeEnd).compose(Field((), ShapeEnd))
//! }
//! ```
//!
//! In addition, Shape subtypes can compose ADT subtypes
//! by calling LiftAdt and retrying composition with the result.
//!
//! ```
//! use elysian::{adt, Input, Field, Output, ShapeEnd, Run, Then, AdtEnd};
//! use t_funk::{typeclass::category::Compose, op_chain::Done};
//!
//! fn shape_compose_adt() {
//!     let shape = Input((), Field((), Output((), ShapeEnd)));
//!     let adt = Then(Run(()), Then(Run(()), AdtEnd));
//!     assert_eq!(shape.compose(adt), Then(Run(shape), Then(Run(()), Then(Run(()), AdtEnd))))
//! }
//! ```
//!

use t_funk::typeclass::category::{Compose, ComposeT, Id};

use crate::{AdtEnd, Combine, Shape, LiftAdt, LiftAdtT, Run, Then};

// ShapeEnd is the compositional identity

impl<A> Id for Shape<A> {
    type Id = AdtEnd;

    fn id() -> Self::Id {
        AdtEnd
    }
}

// Field

impl<A, B> Compose<Shape<B>> for Shape<A> {
    type Compose = Then<LiftAdtT<Self>, Then<LiftAdtT<Shape<B>>, AdtEnd>>;

    fn compose(self, f: Shape<B>) -> Self::Compose {
        Then(self.lift_adt(), Then(f.lift_adt(), AdtEnd))
    }
}

// Composition with pre-lifted ADT members

// Field

impl<A, B> Compose<Run<B>> for Shape<A>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<B>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<B>>;

    fn compose(self, f: Run<B>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C> Compose<Then<B, C>> for Shape<A>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<B, C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<B, C>>;

    fn compose(self, f: Then<B, C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D> Compose<Combine<B, C, D>> for Shape<A>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<B, C, D>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<B, C, D>>;

    fn compose(self, f: Combine<B, C, D>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A> Compose<AdtEnd> for Shape<A>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<AdtEnd>,
{
    type Compose = ComposeT<LiftAdtT<Self>, AdtEnd>;

    fn compose(self, f: AdtEnd) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}
