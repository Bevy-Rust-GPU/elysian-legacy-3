//! Transformations between ADT types
//!
//! End acts as identity for composition
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_nil_compose() {
//!     assert_eq!(End.compose(End), End);
//!     assert_eq!(End.compose(Input(())), Input(()));
//!     assert_eq!(End.compose(Field(())), Field(()));
//!     assert_eq!(End.compose(Output(())), Output(()));
//!     assert_eq!(End.compose(Then(Input(()), End)), Then(Input(()), End));
//!     assert_eq!(End.compose(Combine((), (), ())), Combine((), (), ()));
//! }
//! ```
//!
//! Input can compose End, Input, Field, or Combine
//!
//! ```
//! use elysian::{End, Input, Field, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_input_compose() {
//!     assert_eq!(Input(()).compose(End), Input(()));
//!     assert_eq!(Input(()).compose(Input(())), Then(Input(()), Then(Input(()), End)));
//!     assert_eq!(Input(()).compose(Field(())), Then(Input(()), Then(Field(()), End)));
//!     assert_eq!(Input(()).compose(Combine((), (), ())), Then(Input(()), Then(Combine((), (), ()), End)));
//! }
//! ```
//!
//! Other compositions will fail to compile
//!
//! ```compile_fail,E0277
//! use elysian::{Input, Output};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_input_compose_output() {
//!     Input(()).compose(Output(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Input, Modify};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_input_compose_modify() {
//!     Input(()).compose(Modify(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{End, Input, Then};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_input_compose_sequence() {
//!     Input(()).compose(Then((), End));
//! }
//! ```
//!
//! Field can compose End, Output or Modify
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_field_compose() {
//!     assert_eq!(Field(()).compose(End), Field(()));
//!     assert_eq!(Field(()).compose(Output(())), Then(Field(()), Then(Output(()), End)));
//!     assert_eq!(Field(()).compose(Modify(())), Then(Field(()), Then(Modify(()), End)));
//! }
//! ```
//!
//! Other compositions will fail to compile
//!
//! ```compile_fail,E0277
//! use elysian::{Field, Input};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_field_compose_input() {
//!     Field(()).compose(Input(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Field};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_field_compose_field() {
//!     Field(()).compose(Field(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{End, Field, Then};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_field_compose_sequence() {
//!     Field(()).compose(Then((), End));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Field, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_field_compose_combine() {
//!     Field(()).compose(Combine((), (), ()));
//! }
//! ```
//!
//! Output can compose End, Output or Modify
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_output_compose() {
//!     assert_eq!(Output(()).compose(End), Output(()));
//!     assert_eq!(Output(()).compose(Output(())), Then(Output(()), Then(Output(()), End)));
//!     assert_eq!(Output(()).compose(Modify(())), Then(Output(()), Then(Modify(()), End)));
//! }
//! ```
//!
//! Other compositions will fail to compile
//!
//! ```compile_fail,E0277
//! use elysian::{Output, Input};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_output_compose_input() {
//!     Output(()).compose(Input(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Output, Field};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_output_compose_field() {
//!     Output(()).compose(Field(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{End, Output, Then};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_output_compose_sequence() {
//!     Output(()).compose(Then((), End));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Output, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_output_compose_combine() {
//!     Output(()).compose(Combine((), (), ()));
//! }
//! ```
//!
//! Modify can compose End, Input, Field, Modify, Then or Combine
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_modify_compose() {
//!     assert_eq!(Modify(()).compose(End), Modify(()));
//!     assert_eq!(Modify(()).compose(Input(())), Then(Modify(()), Then(Input(()), End)));
//!     assert_eq!(Modify(()).compose(Field(())), Then(Modify(()), Then(Field(()), End)));
//!     assert_eq!(Modify(()).compose(Modify(())), Then(Modify(()), Then(Modify(()), End)));
//!     assert_eq!(Modify(()).compose(Then(Input(()), End)), Then(Modify(()), Then(Input(()), End)));
//!     assert_eq!(Modify(()).compose(Combine((), (), ())), Then(Modify(()), Then(Combine((), (), ()), End)));
//! }
//! ```
//!
//! Other compositions will fail to compile
//!
//! ```compile_fail,E0277
//! use elysian::{Modify, Output};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_modify_compose_output() {
//!     Modify(()).compose(Output(()));
//! }
//! ```
//!
//! Then can compose anything
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_sequence_compose() {
//!     assert_eq!(Then(Modify(()), End).compose(End), Then(Modify(()), End));
//!     assert_eq!(Then(Modify(()), End).compose(Input(())), Then(Modify(()), Then(Input(()), End)));
//!     assert_eq!(Then(Modify(()), End).compose(Field(())), Then(Modify(()), Then(Field(()), End)));
//!     assert_eq!(Then(Output(()), End).compose(Output(())), Then(Output(()), Then(Output(()), End)));
//!     assert_eq!(Then(Modify(()), End).compose(Modify(())), Then(Modify(()), Then(Modify(()), End)));
//!     assert_eq!(Then(Modify(()), End).compose(Then(Modify(()), End)), Then(Modify(()), Then(Modify(()), End)));
//!     assert_eq!(Then(Modify(()), End).compose(Combine((), (), ())), Then(Modify(()), Then(Combine((), (), ()), End)));
//! }
//! ```
//!
//! Combine can compose End, Output, Modify and Then
//!
//! ```
//! use elysian::{End, Input, Field, Output, Modify, Then, Combine};
//! use t_funk::typeclass::category::Compose;
//!
//! fn test_combine_compose() {
//!     assert_eq!(Combine((), (), ()).compose(End), Combine((), (), ()));
//!     assert_eq!(Combine((), (), ()).compose(Output(())), Then(Combine((), (), ()), Then(Output(()), End)));
//!     assert_eq!(Combine((), (), ()).compose(Modify(())), Then(Combine((), (), ()), Then(Modify(()), End)));
//!     assert_eq!(Combine((), (), ()).compose(Then(Modify(()), End)), Then(Combine((), (), ()), Then(Modify(()), End)));
//! }
//! ```
//!
//! Other compositions will fail to compile
//!
//! ```compile_fail,E0277
//! use elysian::{Combine, Input};
//! use t_funk::typeclass::category::Compose;
//! fn test_combine_compose_input() {
//!     Combine((), (), ()).compose(Input(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::{Combine, Field};
//! use t_funk::typeclass::category::Compose;
//! fn test_combine_compose_field() {
//!     Combine((), (), ()).compose(Field(()));
//! }
//! ```
//! ```compile_fail,E0277
//! use elysian::Combine;
//! use t_funk::typeclass::category::Compose;
//! fn test_combine_compose_combine() {
//!     Combine((), (), ()).compose(Combine((), (), ()));
//! }
//! ```
//!

use t_funk::{
    macros::impl_adt,
    typeclass::category::{Compose, ComposeT},
};

use crate::{Combine, End, Field, Input, Modify, NotEnd, Output, Then};

// Compose End impls

impl_adt! {
    impl<A, B, C> Compose<End> for End | Input<A> | Field<A> | Output<A> | Modify<A> |  Combine<A, B, C> {
        type Compose = Self;

        fn compose(self, _: End) -> Self::Compose {
            self
        }
    }
}

// End

impl<A> Compose<Input<A>> for End {
    type Compose = Input<A>;

    fn compose(self, f: Input<A>) -> Self::Compose {
        f
    }
}

impl<A> Compose<Field<A>> for End {
    type Compose = Field<A>;

    fn compose(self, f: Field<A>) -> Self::Compose {
        f
    }
}

impl<A> Compose<Output<A>> for End {
    type Compose = Output<A>;

    fn compose(self, f: Output<A>) -> Self::Compose {
        f
    }
}

impl<A> Compose<Modify<A>> for End {
    type Compose = Modify<A>;

    fn compose(self, f: Modify<A>) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Then<A, B>> for End
where
    Self: Compose<A>,
{
    type Compose = Then<A, B>;

    fn compose(self, f: Then<A, B>) -> Self::Compose {
        f
    }
}

impl<A, B, C> Compose<Combine<A, B, C>> for End {
    type Compose = Combine<A, B, C>;

    fn compose(self, f: Combine<A, B, C>) -> Self::Compose {
        f
    }
}

// Input

impl<A, B> Compose<Input<B>> for Input<A> {
    type Compose = Then<Self, Then<Input<B>, End>>;

    fn compose(self, f: Input<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B> Compose<Field<B>> for Input<A> {
    type Compose = Then<Self, Then<Field<B>, End>>;

    fn compose(self, f: Field<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Input<A> {
    type Compose = Then<Self, Then<Combine<B, C, F>, End>>;

    fn compose(self, f: Combine<B, C, F>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

// Field

impl<A, B> Compose<Output<B>> for Field<A> {
    type Compose = Then<Self, Then<Output<B>, End>>;

    fn compose(self, f: Output<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B> Compose<Modify<B>> for Field<A> {
    type Compose = Then<Self, Then<Modify<B>, End>>;

    fn compose(self, f: Modify<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

// Output

impl<A, B> Compose<Output<B>> for Output<A> {
    type Compose = Then<Self, Then<Output<B>, End>>;

    fn compose(self, f: Output<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B> Compose<Modify<B>> for Output<A> {
    type Compose = Then<Self, Then<Modify<B>, End>>;

    fn compose(self, f: Modify<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

// Modify

impl<A, B> Compose<Input<B>> for Modify<A> {
    type Compose = Then<Self, Then<Input<B>, End>>;

    fn compose(self, f: Input<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B> Compose<Field<B>> for Modify<A> {
    type Compose = Then<Self, Then<Field<B>, End>>;

    fn compose(self, f: Field<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B> Compose<Modify<B>> for Modify<A> {
    type Compose = Then<Self, Then<Modify<B>, End>>;

    fn compose(self, f: Modify<B>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

impl<A, B, C> Compose<Then<B, C>> for Modify<A>
where
    Self: Compose<B>,
{
    type Compose = Then<Self, Then<B, C>>;

    fn compose(self, f: Then<B, C>) -> Self::Compose {
        Then(self, f)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Modify<A> {
    type Compose = Then<Self, Then<Combine<B, C, F>, End>>;

    fn compose(self, f: Combine<B, C, F>) -> Self::Compose {
        Then(self, Then(f, End))
    }
}

// Then

impl<A, B, C> Compose<C> for Then<A, B>
where
    B: NotEnd + Compose<C>,
    C: NotEnd,
{
    type Compose = Then<A, B::Compose>;

    fn compose(self, rhs: C) -> Self::Compose {
        Then(self.0, self.1.compose(rhs))
    }
}

impl<A, B> Compose<B> for Then<A, End>
where
    A: Compose<B>,
    B: NotEnd,
{
    type Compose = ComposeT<A, B>;

    fn compose(self, rhs: B) -> Self::Compose {
        self.0.compose(rhs)
    }
}

impl<A> Compose<End> for Then<A, End> {
    type Compose = Self;

    fn compose(self, _: End) -> Self::Compose {
        self
    }
}

// Combine

impl<A, B, F, C> Compose<Output<C>> for Combine<A, B, F> {
    type Compose = Then<Self, Then<Output<C>, End>>;

    fn compose(self, rhs: Output<C>) -> Self::Compose {
        Then(self, Then(rhs, End))
    }
}

impl<A, B, F, C> Compose<Modify<C>> for Combine<A, B, F> {
    type Compose = Then<Self, Then<Modify<C>, End>>;

    fn compose(self, rhs: Modify<C>) -> Self::Compose {
        Then(self, Then(rhs, End))
    }
}

impl<A, B, F, C, D> Compose<Then<C, D>> for Combine<A, B, F>
where
    Self: Compose<C>,
{
    type Compose = Then<Self, Then<C, D>>;

    fn compose(self, rhs: Then<C, D>) -> Self::Compose {
        Then(self, rhs)
    }
}
