use t_funk::macros::impl_adt;

use crate::{Field, Input, Output};

pub trait NotShapeEnd {}

impl_adt! {
    impl<A, B> NotShapeEnd for Input<A, B> | Field<A, B> | Output<A, B> {}
}
