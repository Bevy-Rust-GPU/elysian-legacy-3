use t_funk::macros::{types, functions};

#[functions]
#[types]
pub trait ModifyFunction<D> {
    type Inputs;
    type Function;

    fn modify_function(self) -> Self::Function;
}

