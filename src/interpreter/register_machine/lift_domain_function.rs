use t_funk::{
    collection::set::{LiftContext, LiftContextT},
    macros::{functions, types},
};

use crate::{DomainFunction, FunctionT, InputsT};

/// Given a `Domain` type and a `DomainFunction` type,
/// lift the resulting domain function to read input from a context,
/// and produce a setter function to update that context with computed output
#[functions]
#[types]
pub trait LiftDomainFunction<D> {
    type LiftDomainFunction;

    fn lift_domain_function(self) -> Self::LiftDomainFunction;
}

impl<T, D> LiftDomainFunction<D> for T
where
    T: DomainFunction<D>,
    FunctionT<T, D>: LiftContext<InputsT<T, D>>,
{
    type LiftDomainFunction = LiftContextT<FunctionT<T, D>, InputsT<T, D>>;

    fn lift_domain_function(self) -> Self::LiftDomainFunction {
        self.domain().lift_context()
    }
}
