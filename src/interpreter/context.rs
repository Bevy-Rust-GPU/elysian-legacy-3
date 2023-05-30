use type_fields::t_funk::set::{Get, Set};

use crate::{DistanceF32, GradientF32, PositionF32};

/// Evaluation context containing position, distance and gradient
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct PosDistGrad {
    pub pos: PositionF32,
    pub dist: DistanceF32,
    pub grad: GradientF32,
}

impl Get<PositionF32> for PosDistGrad {
    fn get(self) -> PositionF32 {
        self.pos
    }
}

impl Get<DistanceF32> for PosDistGrad {
    fn get(self) -> DistanceF32 {
        self.dist
    }
}

impl Get<GradientF32> for PosDistGrad {
    fn get(self) -> GradientF32 {
        self.grad
    }
}

impl Set<PositionF32> for PosDistGrad {
    fn set(self, t: PositionF32) -> Self {
        Self { pos: t, ..self }
    }
}

impl Set<DistanceF32> for PosDistGrad {
    fn set(self, t: DistanceF32) -> Self {
        Self { dist: t, ..self }
    }
}

impl Set<GradientF32> for PosDistGrad {
    fn set(self, t: GradientF32) -> Self {
        Self { grad: t, ..self }
    }
}
