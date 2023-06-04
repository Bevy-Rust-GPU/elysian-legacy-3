//! Transformations between ADT types

use std::ops::Shr;

use crate::{Combine, Elysian, Field, Input, Modify, Output, Sequence};

// Note: A and B are in reference to the output type flow of a domain operation
// ex:
//     In the distance domain, A is position and B is distance
//     In the gradient domain, A is position and B is gradient

// Input (A -> A) can compose anything that takes an A
// i.e. Input, Field or Combine

impl<A, B> Shr<Input<B>> for Input<A> {
    type Output = Sequence<Self, Input<B>>;

    fn shr(self, rhs: Input<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B> Shr<Field<B>> for Input<A> {
    type Output = Sequence<Self, Field<B>>;

    fn shr(self, rhs: Field<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B, C, F> Shr<Combine<B, C, F>> for Input<A> {
    type Output = Sequence<Self, Combine<B, C, F>>;

    fn shr(self, rhs: Combine<B, C, F>) -> Self::Output {
        Sequence(self, rhs)
    }
}

// Field (A -> B) can compose anything that takes a B
// i.e. Output

impl<A, B> Shr<Output<B>> for Field<A> {
    type Output = Sequence<Self, Output<B>>;

    fn shr(self, rhs: Output<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

// Output (B -> B) can compose anything that takes a B
// i.e. Output
// or a Modify, which occurs after B has been written into C

impl<A, B> Shr<Output<B>> for Output<A> {
    type Output = Sequence<Self, Output<B>>;

    fn shr(self, rhs: Output<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B> Shr<Modify<B>> for Output<A> {
    type Output = Sequence<Self, Modify<B>>;

    fn shr(self, rhs: Modify<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

// Modify (C -> C) can compose another Modify,
// or break an A out of C to begin an Input / Combine chain
impl<A, B> Shr<Modify<B>> for Modify<A> {
    type Output = Sequence<Self, Modify<B>>;

    fn shr(self, rhs: Modify<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B> Shr<Input<B>> for Modify<A> {
    type Output = Sequence<Self, Input<B>>;

    fn shr(self, rhs: Input<B>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B, C, F> Shr<Combine<B, C, F>> for Modify<A> {
    type Output = Sequence<Self, Combine<B, C, F>>;

    fn shr(self, rhs: Combine<B, C, F>) -> Self::Output {
        Sequence(self, rhs)
    }
}

// Sequence (A -> B) defers to its innermost type

impl<A, B, C> Shr<C> for Sequence<A, B>
where
    B: Shr<C>,
{
    type Output = Sequence<A, B::Output>;

    fn shr(self, rhs: C) -> Self::Output {
        Sequence(self.0, self.1 >> rhs)
    }
}

// Combine (A -> B) can compose anything that takes a B
// i.e. Output
// or a Modify, which occurs after B has been written back into C

impl<A, B, F, C> Shr<Output<C>> for Combine<A, B, F> {
    type Output = Sequence<Self, Output<C>>;

    fn shr(self, rhs: Output<C>) -> Self::Output {
        Sequence(self, rhs)
    }
}

impl<A, B, F, C> Shr<Modify<C>> for Combine<A, B, F> {
    type Output = Sequence<Self, Modify<C>>;

    fn shr(self, rhs: Modify<C>) -> Self::Output {
        Sequence(self, rhs)
    }
}

pub trait Compose<T> {
    type Compose;

    fn compose(self, t: T) -> Self::Compose;
}

impl<T, U> Compose<U> for T
where
    T: Elysian + Shr<U>,
{
    type Compose = T::Output;

    fn compose(self, t: U) -> Self::Compose {
        self.shr(t)
    }
}
