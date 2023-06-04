use t_funk::{
    closure::{Curry2, Curry2B},
    collection::set::SetF,
};

use crate::{LiftParam, Set};

impl<T> LiftParam for Set<T> {
    type LiftParam = Curry2B<SetF, T>;

    fn lift_param(self) -> Self::LiftParam {
        SetF.suffix2(self.0)
    }
}
