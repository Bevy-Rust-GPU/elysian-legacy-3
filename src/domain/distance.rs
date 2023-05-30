//! Distance domain
//! Subdomain of position

use std::ops::Neg;

use type_fields::macros::{
    applicative::Applicative, functor::Functor, monad::Monad, Copointed, Pointed,
};

use crate::{DomainF, DomainT};

// Distance domain values
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Pointed,
    Copointed,
    Functor,
    Applicative,
    Monad,
)]
pub struct Distance<T>(pub T);

impl<T> Neg for Distance<T>
where
    T: Neg,
{
    type Output = Distance<T::Output>;

    fn neg(self) -> Self::Output {
        Distance(self.0.neg())
    }
}

pub type DistanceF32 = Distance<f32>;

pub type DistanceT<T> = DomainT<T, Distance<f32>>;
pub type DistanceF = DomainF<Distance<f32>>;
