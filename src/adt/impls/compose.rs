//! Transformations between ADT types

use t_funk::typeclass::category::Compose;

use crate::{Combine, Nil, NotNil, Sequence, Unit};

// Note: A and B are in reference to the output type flow of a domain operation
// ex:
//     In the distance domain, A is position and B is distance
//     In the gradient domain, A is position and B is gradient

// Modify (C -> C) can compose another Modify,
// or break an A out of C to begin an Input / Combine chain

impl<A> Compose<Nil> for Unit<A> {
    type Compose = Self;

    fn compose(self, _: Nil) -> Self::Compose {
        self
    }
}

impl<A, B> Compose<Unit<B>> for Unit<A> {
    type Compose = Sequence<Self, Sequence<Unit<B>, Nil>>;

    fn compose(self, rhs: Unit<B>) -> Self::Compose {
        Sequence(self, Sequence(rhs, Nil))
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Unit<A> {
    type Compose = Sequence<Self, Sequence<Combine<B, C, F>, Nil>>;

    fn compose(self, rhs: Combine<B, C, F>) -> Self::Compose {
        Sequence(self, Sequence(rhs, Nil))
    }
}

impl<A, B, C> Compose<Sequence<B, C>> for Unit<A> {
    type Compose = Sequence<Self, Sequence<B, C>>;

    fn compose(self, rhs: Sequence<B, C>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

// Sequence (A -> B) defers to its innermost type

impl<A, B, C> Compose<C> for Sequence<A, B>
where
    B: NotNil + Compose<C>,
    C: NotNil,
{
    type Compose = Sequence<A, B::Compose>;

    fn compose(self, rhs: C) -> Self::Compose {
        Sequence(self.0, self.1.compose(rhs))
    }
}

impl<A, B> Compose<Nil> for Sequence<A, B> {
    type Compose = Self;

    fn compose(self, _: Nil) -> Self::Compose {
        self
    }
}

impl<A, C> Compose<Unit<C>> for Sequence<A, Nil> {
    type Compose = Sequence<A, Sequence<Unit<C>, Nil>>;

    fn compose(self, rhs: Unit<C>) -> Self::Compose {
        Sequence(self.0, Sequence(rhs, Nil))
    }
}

impl<A, B, C> Compose<Sequence<B, C>> for Sequence<A, Nil> {
    type Compose = Sequence<A, Sequence<B, C>>;

    fn compose(self, rhs: Sequence<B, C>) -> Self::Compose {
        Sequence(self.0, rhs)
    }
}

impl<A, B, C, F> Compose<Combine<B, C, F>> for Sequence<A, Nil> {
    type Compose = Sequence<A, Sequence<Combine<B, C, F>, Nil>>;

    fn compose(self, rhs: Combine<B, C, F>) -> Self::Compose {
        Sequence(self.0, Sequence(rhs, Nil))
    }
}

// Combine (A -> B) can compose anything that takes a B
// i.e. Output
// or a Modify, which occurs after B has been written back into C

impl<A, B, C> Compose<Nil> for Combine<A, B, C> {
    type Compose = Self;

    fn compose(self, _: Nil) -> Self::Compose {
        self
    }
}

impl<A, B, F, C> Compose<Unit<C>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Sequence<Unit<C>, Nil>>;

    fn compose(self, rhs: Unit<C>) -> Self::Compose {
        Sequence(self, Sequence(rhs, Nil))
    }
}

impl<A, B, F, C, D> Compose<Sequence<C, D>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Sequence<C, D>>;

    fn compose(self, rhs: Sequence<C, D>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl<A, B, F, C, D, G> Compose<Combine<C, D, G>> for Combine<A, B, F> {
    type Compose = Sequence<Self, Combine<C, D, G>>;

    fn compose(self, rhs: Combine<C, D, G>) -> Self::Compose {
        Sequence(self, rhs)
    }
}

impl Compose<Nil> for Nil {
    type Compose = Nil;

    fn compose(self, _: Nil) -> Self::Compose {
        self
    }
}

impl<A> Compose<Unit<A>> for Nil {
    type Compose = Unit<A>;

    fn compose(self, f: Unit<A>) -> Self::Compose {
        f
    }
}

impl<A, B> Compose<Sequence<A, B>> for Nil {
    type Compose = Sequence<A, B>;

    fn compose(self, f: Sequence<A, B>) -> Self::Compose {
        f
    }
}

impl<A, B, C> Compose<Combine<A, B, C>> for Nil {
    type Compose = Combine<A, B, C>;

    fn compose(self, f: Combine<A, B, C>) -> Self::Compose {
        f
    }
}
