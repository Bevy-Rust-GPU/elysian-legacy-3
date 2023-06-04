//! Evaluation context containing Position and Distance
use t_funk::collection::set::{Get, Set};

use crate::{Distance, Position};

/// Evaluation context containing position and distance
#[derive(Debug, Default, Copy, Clone)]
pub struct PosDist<T> {
    pub pos: Position<T>,
    pub dist: Distance<T>,
}

impl<T> Get<Position<T>> for PosDist<T> {
    fn get(self) -> Position<T> {
        self.pos
    }
}

impl<T> Get<Distance<T>> for PosDist<T> {
    fn get(self) -> Distance<T> {
        self.dist
    }
}

impl<T> Set<Position<T>> for PosDist<T> {
    fn set(self, t: Position<T>) -> Self {
        Self { pos: t, ..self }
    }
}

impl<T> Set<Distance<T>> for PosDist<T> {
    fn set(self, t: Distance<T>) -> Self {
        Self { dist: t, ..self }
    }
}
