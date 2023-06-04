use crate::{Combine, Field, Input, Output, Sequence, Modify};

pub trait LiftAdt {
    type LiftAdt;

    fn adt(self) -> Self::LiftAdt;
}

pub type LiftAdtT<T> = <T as LiftAdt>::LiftAdt;

impl<T> LiftAdt for Input<T> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}

impl<T> LiftAdt for Field<T> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}

impl<T> LiftAdt for Output<T> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}

impl<T> LiftAdt for Modify<T> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}

impl<A, B> LiftAdt for Sequence<A, B> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}

impl<A, B, F> LiftAdt for Combine<A, B, F> {
    type LiftAdt = Self;

    fn adt(self) -> Self::LiftAdt {
        self
    }
}
