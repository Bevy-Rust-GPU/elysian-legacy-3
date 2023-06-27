/// Evaluation context containing Position, Distance, Gradient and Color
use t_funk::macros::set::set;

use crate::{Color, Distance, Gradient, PosDistGrad, Position};

#[derive(Debug, Default, Copy, Clone)]
#[set]
pub struct PosDistGradColor<P, D, G, C> {
    #[set = [P = Position::<P>, D = Distance::<D>, G = Gradient::<G>]]
    pub pos_dist_grad: PosDistGrad<P, D, G>,
    #[newtype = Color::<C>]
    pub color: C,
}

