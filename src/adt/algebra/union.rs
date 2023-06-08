use t_funk::{
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{symbol::Union as UnionS, Combine, LiftAdtF, Then, Field, Input, Output, Modify};

pub fn union() -> OpChain<LiftAdtF, UnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait Union<T> {
    type Union;

    fn union(self, rhs: T) -> Self::Union;
}

impl_adt! {
    impl<A, B, C, R> Union<R> for Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C> {
        type Union = Combine<Self, R, UnionS>;

        fn union(self, rhs: R) -> Self::Union {
            Combine(self, rhs, UnionS)
        }
    }
}
