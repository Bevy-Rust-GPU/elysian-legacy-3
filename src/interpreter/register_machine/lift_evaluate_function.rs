use t_funk::{
    collection::set::{LiftContext, LiftContextT},
    macros::{functions, types},
};

use crate::{EvaluateFunction, FunctionT, InputsT};

/// Given a `Domain` type and a `DomainFunction` type,
/// lift the resulting domain function to read input from a context,
/// and produce a setter function to update that context with computed output
#[functions]
#[types]
pub trait LiftEvaluateFunction<D> {
    type LiftEvaluateFunction;

    fn lift_evaluate_function(self) -> Self::LiftEvaluateFunction;
}

impl<T, D> LiftEvaluateFunction<D> for T
where
    T: EvaluateFunction<D>,
    FunctionT<T, D>: LiftContext<InputsT<T, D>>,
{
    type LiftEvaluateFunction = LiftContextT<FunctionT<T, D>, InputsT<T, D>>;

    fn lift_evaluate_function(self) -> Self::LiftEvaluateFunction {
        self.evaluate_function().lift_context()
    }
}
