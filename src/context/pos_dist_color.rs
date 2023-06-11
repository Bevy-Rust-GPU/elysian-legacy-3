/// Evaluation context containing Position, Distance and Color
use t_funk::collection::set::{Get, Set};

use crate::{Color, Distance, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
pub struct PosDistColor<P, D, C> {
    pub pos_dist: PosDist<P, D>,
    pub color: Color<C>,
}

impl<P, D, G> Get<Position<P>> for PosDistColor<P, D, G> {
    fn get(self) -> Position<P> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Distance<D>> for PosDistColor<P, D, G> {
    fn get(self) -> Distance<D> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Color<G>> for PosDistColor<P, D, G> {
    fn get(self) -> Color<G> {
        self.color
    }
}

impl<P, D, G> Set<Position<P>> for PosDistColor<P, D, G> {
    fn set(self, t: Position<P>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<P, D, G> Set<Distance<D>> for PosDistColor<P, D, G> {
    fn set(self, t: Distance<D>) -> Self {
        Self {
            pos_dist: self.pos_dist.set(t),
            ..self
        }
    }
}

impl<P, D, G> Set<Color<G>> for PosDistColor<P, D, G> {
    fn set(self, t: Color<G>) -> Self {
        Self { color: t, ..self }
    }
}

