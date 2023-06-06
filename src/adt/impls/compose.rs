//! Transformations between ADT types

use t_funk::typeclass::category::Compose;

use crate::{Combine, Sequence, Unit};

// Note: A and B are in reference to the output type flow of a domain operation
// ex:
//     In the distance domain, A is position and B is distance
//     In the gradient domain, A is position and B is gradient

// Modify (C -> C) can compose another Modify,
// or break an A out of C to begin an Input / Combine chain
impl<A, B> Compose<Unit<B>> for Unit<A> {
    type Compose = Sequence<Self, Unit<B>>;

    fn compose(self, rhs: Unit<B>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Unit<A> {
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

impl<A, B, F, C> Compose<Unit<C>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Unit<C>>;

    fn compose(self, rhs: Unit<C>) -> Self::Compose {
        Sequence(self, rhs)
    }
}
