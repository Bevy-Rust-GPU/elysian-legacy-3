use t_funk::macros::{impl_adt, types};

use crate::{Combine, Field, Input, Output, Sequence, Unit};

#[types]
pub trait LiftCombine {
    type LiftCombine;

    fn lift_combine(self) -> Self::LiftCombine;
}

impl_adt! {
    impl<A, B> LiftCombine for Input<A, B> | Field<A, B> | Output<A, B> {
        type LiftCombine = Self;

        fn lift_combine(self) -> Self::LiftCombine {
            self
        }
    }
}

impl_adt! {
    impl<T> LiftCombine for Unit<T> {
        type LiftCombine = Self;

        fn lift_combine(self) -> Self::LiftCombine {
            self
        }
    }
}

impl<A, B> LiftCombine for Sequence<A, B>
where
    A: LiftCombine,
    B: LiftCombine,
{
    type LiftCombine = Sequence<LiftCombineT<A>, LiftCombineT<B>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Sequence(self.0.lift_combine(), self.1.lift_combine())
    }
}

impl<A, B, F> LiftCombine for Combine<A, B, F>
where
    A: LiftCombine,
    B: LiftCombine,
    F: LiftCombine,
{
    type LiftCombine = Combine<LiftCombineT<A>, LiftCombineT<B>, LiftCombineT<F>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Combine(
            self.0.lift_combine(),
            self.1.lift_combine(),
            self.2.lift_combine(),
        )
    }
}
