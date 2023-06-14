//! Evaluation context containing Position and Distance
use t_funk::macros::set::set;

use crate::{Distance, Position};

/// Evaluation context containing position and distance
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[set]
pub struct PosDist<P, D> {
    #[newtype = Position::<P>]
    pub pos: P,
    #[newtype = Distance::<D>]
    pub dist: D,
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::collection::set::{Empty, Insert, Subtraction, Union};

    use crate::{Distance, PosDist, Position};

    #[test]
    fn test_pos_dist_set() {
        let empty = PosDist::<Vec2, f32>::empty();
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
