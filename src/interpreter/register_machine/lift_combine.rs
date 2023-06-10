use t_funk::macros::{impl_adt, types};

use crate::{AdtEnd, Combine, Field, Input, Modify, Output, Run, ShapeEnd, Then};

#[types]
pub trait LiftCombine<D> {
    type LiftCombine;

    fn lift_combine(self) -> Self::LiftCombine;
}

impl_adt! {
    impl<A, B, D> LiftCombine<D> for Input<A, B> | Field<A, B> | Output<A, B> | ShapeEnd {
        type LiftCombine = Self;

        fn lift_combine(self) -> Self::LiftCombine {
            self
        }
    }
}

impl<A, D> LiftCombine<D> for Modify<A> {
    type LiftCombine = Self;

    fn lift_combine(self) -> Self::LiftCombine {
        self
    }
}

impl_adt! {
    impl<A, D> LiftCombine<D> for Run<A> | AdtEnd {
        type LiftCombine = Self;

        fn lift_combine(self) -> Self::LiftCombine {
            self
        }
    }
}

impl<A, B, D> LiftCombine<D> for Then<A, B>
where
    A: LiftCombine<D>,
    B: LiftCombine<D>,
{
    type LiftCombine = Then<LiftCombineT<A, D>, LiftCombineT<B, D>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Then(self.0.lift_combine(), self.1.lift_combine())
    }
}

impl<A, B, F, D> LiftCombine<D> for Combine<A, B, F>
where
    A: LiftCombine<D>,
    B: LiftCombine<D>,
    F: LiftCombine<D>,
{
    type LiftCombine = Combine<LiftCombineT<A, D>, LiftCombineT<B, D>, LiftCombineT<F, D>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Combine(
            self.0.lift_combine(),
            self.1.lift_combine(),
            self.2.lift_combine(),
        )
    }
}
