/// Evaluation context containing Position, Distance and Gradient
use t_funk::collection::set::{Get, Insert, Remove};

use crate::{Distance, Gradient, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
pub struct PosDistGrad<P, D, G> {
    pub pos_dist: PosDist<P, D>,
    pub grad: G,
}

impl<P, D, G> Get<Position<P>> for PosDistGrad<Position<P>, D, G> {
    fn get(self) -> Position<P> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Distance<D>> for PosDistGrad<P, Distance<D>, G> {
    fn get(self) -> Distance<D> {
        self.pos_dist.get()
    }
}

impl<P, D, G> Get<Gradient<G>> for PosDistGrad<P, D, Gradient<G>> {
    fn get(self) -> Gradient<G> {
        self.grad
    }
}

impl<PA, PB, D, G> Insert<Position<PB>> for PosDistGrad<PA, D, G> {
    type Insert = PosDistGrad<Position<PB>, D, G>;

    fn insert(self, t: Position<PB>) -> Self::Insert {
        let PosDistGrad { pos_dist, grad } = self;

        PosDistGrad {
            pos_dist: pos_dist.insert(t),
            grad,
        }
    }
}

impl<P, DA, DB, G> Insert<Distance<DB>> for PosDistGrad<P, DA, G> {
    type Insert = PosDistGrad<P, Distance<DB>, G>;

    fn insert(self, t: Distance<DB>) -> Self::Insert {
        let PosDistGrad { pos_dist, grad } = self;

        PosDistGrad {
            pos_dist: pos_dist.insert(t),
            grad,
        }
    }
}

impl<P, D, GA, GB> Insert<Gradient<GB>> for PosDistGrad<P, D, GA> {
    type Insert = PosDistGrad<P, D, Gradient<GB>>;

    fn insert(self, grad: Gradient<GB>) -> Self::Insert {
        let PosDistGrad { pos_dist, .. } = self;
        PosDistGrad { grad, pos_dist }
    }
}

impl<P, D, G> Remove<Position<P>> for PosDistGrad<Position<P>, D, G> {
    type Remove = PosDistGrad<(), D, G>;

    fn remove(self) -> (Self::Remove, Position<P>) {
        let PosDistGrad { pos_dist, grad } = self;
        let (pos_dist, p) = pos_dist.remove();
        (PosDistGrad { pos_dist, grad }, p)
    }
}

impl<P, D, G> Remove<Distance<D>> for PosDistGrad<P, Distance<D>, G> {
    type Remove = PosDistGrad<P, (), G>;

    fn remove(self) -> (Self::Remove, Distance<D>) {
        let PosDistGrad { pos_dist, grad } = self;
        let (pos_dist, d) = pos_dist.remove();
        (PosDistGrad { pos_dist, grad }, d)
    }
}

impl<P, D, G> Remove<Gradient<G>> for PosDistGrad<P, D, Gradient<G>> {
    type Remove = PosDistGrad<P, D, ()>;

    fn remove(self) -> (Self::Remove, Gradient<G>) {
        let PosDistGrad { pos_dist, grad } = self;

        (PosDistGrad { pos_dist, grad: () }, grad)
    }
}
