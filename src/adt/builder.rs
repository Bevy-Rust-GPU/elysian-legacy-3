use std::ops::Shr;

use crate::{LiftAdt, LiftAdtT};

pub struct AdtBuilder<T>(T);

impl<T, U> Shr<U> for AdtBuilder<T>
where
    U: LiftAdt,
    T: Shr<LiftAdtT<U>>,
{
    type Output = AdtBuilder<<T as Shr<LiftAdtT<U>>>::Output>;

    fn shr(self, rhs: U) -> Self::Output {
        AdtBuilder(self.0 >> rhs.adt())
    }
}

pub struct Done;

impl<T> Shr<Done> for AdtBuilder<T> {
    type Output = T;

    fn shr(self, _: Done) -> Self::Output {
        self.0
    }
}

pub struct Do;

impl<T> Shr<T> for Do
where
    T: LiftAdt,
{
    type Output = AdtBuilder<LiftAdtT<T>>;

    fn shr(self, rhs: T) -> Self::Output {
        AdtBuilder(rhs.adt())
    }
}
