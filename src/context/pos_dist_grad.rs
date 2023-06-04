/// Evaluation context containing Position, Distance and Gradient
use t_funk::collection::set::{Get, Set};

use crate::{Distance, Gradient, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
pub struct PosDistGrad<T> {
    pub pos_dist: PosDist<T>,
    pub grad: Gradient<T>,
}

impl<T> Get<Position<T>> for PosDistGrad<T> {
    fn get(self) -> Position<T> {
        self.pos_dist.get()
    }
}

impl<T> Get<Distance<T>> for PosDistGrad<T> {
    fn get(self) -> Distance<T> {
        self.pos_dist.get()
    }
}

impl<T> Get<Gradient<T>> for PosDistGrad<T> {
    fn get(self) -> Gradient<T> {
        self.grad
    }
}

impl<T> Set<Position<T>> for PosDistGrad<T> {
    fn set(self, t: Position<T>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<T> Set<Distance<T>> for PosDistGrad<T> {
    fn set(self, t: Distance<T>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<T> Set<Gradient<T>> for PosDistGrad<T> {
    fn set(self, t: Gradient<T>) -> Self {
        Self { grad: t, ..self }
    }
}
