use t_funk::macros::{functions, types};

/// A symbol type that can produce a function corresponding to a `Domain<T>`
#[types]
pub trait EvaluateInputs<T> {
    type Inputs;
    type Moves;
}

/// A symbol type that can produce a function corresponding to a `Domain<T>`
#[functions]
#[types]
pub trait EvaluateFunction<T> {
    type Function;

    fn evaluate_function(self) -> Self::Function;
}
