use crate::{AdtEnd, Combine, LiftAdt, LiftAdtT, Modify, Run, Then};
use t_funk::typeclass::category::{Compose, ComposeT};

impl<A, B> Compose<Modify<B>> for Modify<A>
where
    Self: LiftAdt,
    Modify<B>: LiftAdt,
    LiftAdtT<Self>: Compose<LiftAdtT<Modify<B>>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, LiftAdtT<Modify<B>>>;

    fn compose(self, f: Modify<B>) -> Self::Compose {
        self.lift_adt().compose(f.lift_adt())
    }
}

impl<T, A> Compose<Run<A>> for Modify<T>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Run<A>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Run<A>>;

    fn compose(self, f: Run<A>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<T, A, B> Compose<Then<A, B>> for Modify<T>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Then<A, B>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Then<A, B>>;

    fn compose(self, f: Then<A, B>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}

impl<T, A, B, C> Compose<Combine<A, B, C>> for Modify<T>
where
    Self: LiftAdt,
    LiftAdtT<Self>: Compose<Combine<A, B, C>>,
{
    type Compose = ComposeT<LiftAdtT<Self>, Combine<A, B, C>>;

    fn compose(self, f: Combine<A, B, C>) -> Self::Compose {
        self.lift_adt().compose(f)
    }
}
