use t_funk::macros::{functions, types};

/// Extension trait over `EvaluateFunctions<D>`.
/// Describes desired inputs, and which of its inputs should be supplied by-move.
#[types]
pub trait EvaluateInputs<D>: EvaluateFunction<D> {
    type Inputs;
    type Moves;
}

/// A symbol type that can produce a function corresponding to a `Domain<T>`
#[functions]
#[types]
pub trait EvaluateFunction<D> {
    type Function;

    fn evaluate_function(self) -> Self::Function;
}
