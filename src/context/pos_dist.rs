//! Evaluation context containing Position and Distance
use t_funk::collection::set::{
    Drop, DropT, Empty, Get, Insert, InsertT, Remove, SubtractFrom, UnionWith,
};

use crate::{Distance, Position};

/// Evaluation context containing position and distance
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosDist<P, D> {
    pub pos: P,
    pub dist: D,
}

impl<P, D> Get<Position<P>> for PosDist<Position<P>, D> {
    fn get(self) -> Position<P> {
        self.pos
    }
}

impl<P, D> Get<Distance<D>> for PosDist<P, Distance<D>> {
    fn get(self) -> Distance<D> {
        self.dist
    }
}

impl<PA, PB, D> Insert<Position<PB>> for PosDist<PA, D> {
    type Insert = PosDist<Position<PB>, D>;

    fn insert(self, pos: Position<PB>) -> Self::Insert {
        let PosDist { dist, .. } = self;

        PosDist { pos, dist }
    }
}

impl<P, DA, DB> Insert<Distance<DB>> for PosDist<P, DA> {
    type Insert = PosDist<P, Distance<DB>>;

    fn insert(self, dist: Distance<DB>) -> Self::Insert {
        let PosDist { pos, .. } = self;
        PosDist { pos, dist }
    }
}

impl<P, D> Remove<Position<P>> for PosDist<Position<P>, D> {
    type Remove = PosDist<(), D>;

    fn remove(self) -> (Self::Remove, Position<P>) {
        let PosDist { pos, dist } = self;
        (PosDist { pos: (), dist }, pos)
    }
}

impl<P, D> Remove<Distance<D>> for PosDist<P, Distance<D>> {
    type Remove = PosDist<P, ()>;

    fn remove(self) -> (Self::Remove, Distance<D>) {
        let PosDist { pos, dist } = self;
        (PosDist { pos, dist: () }, dist)
    }
}

impl<P, D> Empty for PosDist<P, D> {
    type Empty = PosDist<(), ()>;

    fn empty(self) -> Self::Empty {
        PosDist { pos: (), dist: () }
    }
}

impl<P, D, U> UnionWith<U> for PosDist<P, D>
where
    U: Insert<P>,
    InsertT<U, P>: Insert<D>,
{
    type UnionWith = InsertT<InsertT<U, P>, D>;

    fn union_with(self, u: U) -> Self::UnionWith {
        u.insert(self.pos).insert(self.dist)
    }
}

impl<P, D, U> SubtractFrom<U> for PosDist<P, D>
where
    U: Drop<P>,
    DropT<U, P>: Drop<D>,
{
    type SubtractFrom = DropT<DropT<U, P>, D>;

    fn subtract_from(self, u: U) -> Self::SubtractFrom {
        u.drop().drop()
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::collection::set::{Empty, Insert, Subtraction, Union};

    use crate::{Distance, PosDist, Position};

    #[test]
    fn test_pos_dist_set() {
        let empty = PosDist::<Vec2, f32>::default().empty();
        assert_eq!(empty, PosDist { pos: (), dist: () });

        let set_a = empty.insert(Position(Vec2::ZERO));
        assert_eq!(
            set_a,
            PosDist {
                pos: Position(Vec2::ZERO),
                dist: ()
            }
        );

        let set_b = empty.insert(Distance(0.0));
        assert_eq!(
            set_b,
            PosDist {
                pos: (),
                dist: Distance(0.0)
            }
        );

        let union = set_a.union(set_b);
        assert_eq!(
            union,
            PosDist {
                pos: Position(Vec2::ZERO),
                dist: Distance(0.0)
            }
        );

        let sub_a = union.subtraction(set_a);
        assert_eq!(
            sub_a,
            PosDist {
                pos: (),
                dist: Distance(0.0)
            }
        );

        let sub_b = union.subtraction(set_b);
        assert_eq!(
            sub_b,
            PosDist {
                pos: Position(Vec2::ZERO),
                dist: ()
            }
        );

        let empty = union.subtraction(union);
        assert_eq!(empty, PosDist { pos: (), dist: () });
    }
}
