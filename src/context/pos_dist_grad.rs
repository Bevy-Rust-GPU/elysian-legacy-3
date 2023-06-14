/// Evaluation context containing Position, Distance and Gradient
use t_funk::macros::set::set;

use crate::{Distance, Gradient, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
#[set]
pub struct PosDistGrad<P, D, G> {
    #[set = [P = Position::<P>, D = Distance::<D>]]
    pub pos_dist: PosDist<P, D>,
    #[newtype = Gradient::<G>]
    pub grad: G,
}
