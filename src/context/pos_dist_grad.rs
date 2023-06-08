/// Evaluation context containing Position, Distance and Gradient
use t_funk::collection::set::{Get, Set};

use crate::{Distance, Gradient, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
pub struct PosDistGrad<P, D, G> {
    pub pos_dist: PosDist<P, D>,
    pub grad: Gradient<G>,
}

impl<P, D, G> Get<Position<P>> for PosDistGrad<P, D, G> {
    fn get(self) -> Position<P> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Distance<D>> for PosDistGrad<P, D, G> {
    fn get(self) -> Distance<D> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Gradient<G>> for PosDistGrad<P, D, G> {
    fn get(self) -> Gradient<G> {
        self.grad
    }
}

impl<P, D, G> Set<Position<P>> for PosDistGrad<P, D, G> {
    fn set(self, t: Position<P>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<P, D, G> Set<Distance<D>> for PosDistGrad<P, D, G> {
    fn set(self, t: Distance<D>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<P, D, G> Set<Gradient<G>> for PosDistGrad<P, D, G> {
    fn set(self, t: Gradient<G>) -> Self {
        Self { grad: t, ..self }
    }
}
