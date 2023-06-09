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

use t_funk::{
    macros::impl_adt,
    typeclass::category::{Compose, ComposeT, Id},
};

use crate::{
    AdtEnd, Combine, Field, Input, LiftAdt, LiftAdtT, NotShapeEnd, Output, Run, ShapeEnd, Then,
};

// ShapeEnd is the compositional identity

impl_adt! {
    impl<A, B> Id for ShapeEnd | Input<A, B> | Field<A, B> | Output<A, B> {
        type Id = ShapeEnd;

        fn id() -> Self::Id {
            ShapeEnd
        }
    }
}

impl Compose<ShapeEnd> for ShapeEnd {
    type Compose = ShapeEnd;

    fn compose(self, f: ShapeEnd) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Input<A, B>> for ShapeEnd {
    type Compose = Input<A, B>;

    fn compose(self, f: Input<A, B>) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Field<A, B>> for ShapeEnd {
    type Compose = Field<A, B>;

    fn compose(self, f: Field<A, B>) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Output<A, B>> for ShapeEnd {
    type Compose = Output<A, B>;

    fn compose(self, f: Output<A, B>) -> Self::Compose {
        f
    }
}

// Composition with ShapeEnd

impl_adt! {
    impl<A, B> Compose<ShapeEnd> for Input<A, B> | Field<A, B> | Output<A, B>
    {
        type Compose = Self;

        fn compose(self, _: ShapeEnd) -> Self::Compose {
            self
        }
    }
}

// Recursive cases

impl_adt! {
    impl<A, B, C> Compose<C> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        B: NotShapeEnd + Compose<C>,
        C: NotShapeEnd,
    {
        type Compose = This<A, ComposeT<B, C>>;

        fn compose(self, f: C) -> Self::Compose {
            This(self.0, self.1.compose(f))
        }
    }
}

// Input

impl<A, B, C> Compose<Input<B, C>> for Input<A, ShapeEnd> {
    type Compose = Input<A, Input<B, C>>;

    fn compose(self, f: Input<B, C>) -> Self::Compose {
        Input(self.0, f)
    }
}

impl<A, B, C> Compose<Field<B, C>> for Input<A, ShapeEnd> {
    type Compose = Input<A, Field<B, C>>;

    fn compose(self, f: Field<B, C>) -> Self::Compose {
        Input(self.0, f)
    }
}

// Field

impl<A, B, C> Compose<Output<B, C>> for Field<A, ShapeEnd> {
    type Compose = Field<A, Output<B, C>>;

    fn compose(self, f: Output<B, C>) -> Self::Compose {
        Field(self.0, f)
    }
}

// Output

impl<A, B, C> Compose<Output<B, C>> for Output<A, ShapeEnd> {
    type Compose = Output<A, Output<B, C>>;

    fn compose(self, f: Output<B, C>) -> Self::Compose {
        Output(self.0, f)
    }
}

// Composing with already-lifted ADT members

// Input

impl<A, B, C> Compose<Run<C>> for Input<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<C>>;

    fn compose(self, f: Run<C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D> Compose<Then<C, D>> for Input<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<C, D>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<C, D>>;

    fn compose(self, f: Then<C, D>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D, E> Compose<Combine<C, D, E>> for Input<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<C, D, E>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<C, D, E>>;

    fn compose(self, f: Combine<C, D, E>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B> Compose<AdtEnd> for Input<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<AdtEnd>,
{
    type Compose = ComposeT<LiftAdtT<Self>, AdtEnd>;

    fn compose(self, f: AdtEnd) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

// Field

impl<A, B, C> Compose<Run<C>> for Field<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<C>>;

    fn compose(self, f: Run<C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D> Compose<Then<C, D>> for Field<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<C, D>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<C, D>>;

    fn compose(self, f: Then<C, D>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D, E> Compose<Combine<C, D, E>> for Field<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<C, D, E>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<C, D, E>>;

    fn compose(self, f: Combine<C, D, E>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B> Compose<AdtEnd> for Field<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<AdtEnd>,
{
    type Compose = ComposeT<LiftAdtT<Self>, AdtEnd>;

    fn compose(self, f: AdtEnd) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

// Output

impl<A, B, C> Compose<Run<C>> for Output<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<C>>;

    fn compose(self, f: Run<C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D> Compose<Then<C, D>> for Output<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<C, D>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<C, D>>;

    fn compose(self, f: Then<C, D>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B, C, D, E> Compose<Combine<C, D, E>> for Output<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<C, D, E>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<C, D, E>>;

    fn compose(self, f: Combine<C, D, E>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<A, B> Compose<AdtEnd> for Output<A, B>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<AdtEnd>,
{
    type Compose = ComposeT<LiftAdtT<Self>, AdtEnd>;

    fn compose(self, f: AdtEnd) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

// ShapeEnd

impl<C> Compose<Run<C>> for ShapeEnd
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<C>>;

    fn compose(self, f: Run<C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<C, D> Compose<Then<C, D>> for ShapeEnd
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<C, D>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<C, D>>;

    fn compose(self, f: Then<C, D>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<C, D, E> Compose<Combine<C, D, E>> for ShapeEnd
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<C, D, E>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<C, D, E>>;

    fn compose(self, f: Combine<C, D, E>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl Compose<AdtEnd> for ShapeEnd
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<AdtEnd>,
{
    type Compose = ComposeT<LiftAdtT<Self>, AdtEnd>;

    fn compose(self, f: AdtEnd) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}
