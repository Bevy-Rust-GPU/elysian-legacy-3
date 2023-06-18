use t_funk::macros::{functions, types};

#[functions]
#[types]
pub trait Evaluable {
    type Evaluable;
}
