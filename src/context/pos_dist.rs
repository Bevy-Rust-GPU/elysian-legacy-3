//! Evaluation context containing Position and Distance
use t_funk::collection::set::{Get, Set};

use crate::{Distance, Position};

/// Evaluation context containing position and distance
#[derive(Debug, Default, Copy, Clone)]
pub struct PosDist<P, D> {
    pub pos: Position<P>,
    pub dist: Distance<D>,
}

impl<P, D> Get<Position<P>> for PosDist<P, D> {
    fn get(self) -> Position<P> {
        self.pos
    }
}

impl<P, D> Get<Distance<D>> for PosDist<P, D> {
    fn get(self) -> Distance<D> {
        self.dist
    }
}

impl<P, D> Set<Position<P>> for PosDist<P, D> {
    fn set(self, t: Position<P>) -> Self {
        Self { pos: t, ..self }
    }
}

impl<P, D> Set<Distance<D>> for PosDist<P, D> {
    fn set(self, t: Distance<D>) -> Self {
        Self { dist: t, ..self }
    }
}
