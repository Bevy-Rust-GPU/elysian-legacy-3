use t_funk::{
    closure::{Closure, OutputT},
    closure::{Curry2, Curry2B},
    collection::set::SetF,
    typeclass::functor::Fmap,
};

use crate::{LiftAdt, LiftModifier, Modify};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set<T>(pub T);

impl<T, F> Fmap<F> for Set<T>
where
    F: Closure<T>,
{
    type Fmap = Set<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Set(f.call(self.0))
    }
}

impl<T> LiftAdt for Set<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T> LiftModifier for Set<T> {
    type LiftModifier = Curry2B<SetF, T>;

    fn lift_modifier(self) -> Self::LiftModifier {
        SetF.suffix2(self.0)
    }
}
