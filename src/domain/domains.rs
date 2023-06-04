use crate::{Distance, Gradient};

pub type Dist<T> = (Distance<T>, ());
pub type DistGrad<T> = (Distance<T>, (Gradient<T>, ()));
