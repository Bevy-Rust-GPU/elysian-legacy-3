//! Transformations between ADT types

use t_funk::typeclass::category::{Compose, ComposeT};

use crate::{Combine, Modify, Sequence, Shape};

// Note: A and B are in reference to the output type flow of a domain operation
// ex:
//     In the distance domain, A is position and B is distance
//     In the gradient domain, A is position and B is gradient

// Modify (C -> C) can compose another Modify,
// or break an A out of C to begin an Input / Combine chain
impl<A, B> Compose<Shape<B>> for Shape<A>
where
    A: Compose<B>,
{
    type Compose = Shape<ComposeT<A, B>>;

    fn compose(self, rhs: Shape<B>) -> Self::Compose {
        Shape(self.0.compose(rhs.0))
    }
}

impl<A, B> Compose<Modify<B>> for Shape<A> {
    type Compose = Sequence<Self, Modify<B>>;

    fn compose(self, rhs: Modify<B>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Shape<A> {
    type Compose = Sequence<Self, Combine<B, C, F>>;

    fn compose(self, rhs: Combine<B, C, F>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

// Modify (C -> C) can compose another Modify,
// or break an A out of C to begin an Input / Combine chain
impl<A, B> Compose<Modify<B>> for Modify<A> {
    type Compose = Sequence<Self, Modify<B>>;

    fn compose(self, rhs: Modify<B>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B> Compose<Shape<B>> for Modify<A> {
    type Compose = Sequence<Self, Shape<B>>;

    fn compose(self, rhs: Shape<B>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Modify<A> {
    type Compose = Sequence<Self, Combine<B, C, F>>;

    fn compose(self, rhs: Combine<B, C, F>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

// Sequence (A -> B) defers to its innermost type

impl<A, B, C> Compose<C> for Sequence<A, B>
where
    B: Compose<C>,
{
    type Compose = Sequence<A, B::Compose>;

    fn compose(self, rhs: C) -> Self::Compose {
        Sequence(self.0, self.1.compose(rhs))
    }
}

// Combine (A -> B) can compose anything that takes a B
// i.e. Output
// or a Modify, which occurs after B has been written back into C

impl<A, B, F, C> Compose<Shape<C>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Shape<C>>;

    fn compose(self, rhs: Shape<C>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B, F, C> Compose<Modify<C>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Modify<C>>;

    fn compose(self, rhs: Modify<C>) -> Self::Compose {
        Sequence(self, rhs)
    }
}
