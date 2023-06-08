use crate::{Distance, Gradient};

pub type Dist<D> = (Distance<D>, ());
pub type DistGrad<D, G> = (Distance<D>, (Gradient<G>, ()));
