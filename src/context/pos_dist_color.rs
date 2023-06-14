/// Evaluation context containing Position, Distance and Color
use t_funk::macros::set::set;

use crate::{Color, Distance, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
#[set]
pub struct PosDistColor<P, D, C> {
    #[set = [P = Position::<P>, D = Distance::<D>]]
    pub pos_dist: PosDist<P, D>,
    #[newtype = Color::<C>]
    pub color: C,
}

