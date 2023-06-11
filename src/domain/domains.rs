use crate::{Distance, Gradient, Color};

pub type Dist<D> = (Distance<D>, ());
pub type DistGrad<D, G> = (Distance<D>, (Gradient<G>, ()));
pub type DistColor<D, C> = (Distance<D>, (Color<C>, ()));
