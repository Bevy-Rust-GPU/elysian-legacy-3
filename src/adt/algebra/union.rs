use t_funk::{
    macros::{functions, impl_adt, types},
    r#do::DoUnit,
};

use crate::{symbol::Union as UnionS, Combine, LiftAdtF, Sequence, Unit};

pub fn union() -> DoUnit<LiftAdtF, UnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait Union<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl_adt! {
    impl<A, B, C, R> Union<R> for Unit<A> | Sequence<A, B> | Combine<A, B, C> {
        type Union = Combine<Self, R, UnionS>;

        fn union(self, rhs: R) -> Self::Union {
            Combine(self, rhs, UnionS)
        }
    }
}
