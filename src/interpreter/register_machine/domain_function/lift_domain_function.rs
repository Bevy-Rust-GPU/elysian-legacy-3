use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::{
        hlist::{Cons, Nil},
        set::{LiftContext, LiftContextT},
    },
    macros::{functions, types},
    typeclass::arrow::{Fanout, FanoutT},
};

use crate::{DomainFunction, Field, FunctionT, Input, InputsT, NotShapeEnd, Output, ShapeEnd};

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

/*
impl<T, D> LiftDomainFunction<D> for T
where
    T: DomainFunction<D>,
{
    type LiftDomainFunction = FunctionT<T, D>;

    fn lift_domain_function(self) -> Self::LiftDomainFunction {
        self.domain()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftDomainFunctions<D, N>(PhantomData<D>, N);

trait IsLiftDomainFunctions {}

impl<D, N> IsLiftDomainFunctions for LiftDomainFunctions<D, N> {}

impl<D, N, T> Closure<T> for LiftDomainFunctions<D, N>
where
    T: Clone + LiftDomainFunction<D>,
    LiftDomainFunctionT<T, D>: Fanout<OutputT<N, T>>,
    N: Closure<T>,
    N: IsLiftDomainFunctions,
{
    type Output = Cons<LiftDomainFunctionT<T, D>, OutputT<N, T>>;

    fn call(self, input: T) -> Self::Output {
        Cons(input.clone().lift_domain_function(), self.1.call(input))
    }
}

impl<D, T> Closure<T> for LiftDomainFunctions<D, ()>
where
    T: LiftDomainFunction<D>,
{
    type Output = Cons<LiftDomainFunctionT<T, D>, Nil>;

    fn call(self, input: T) -> Self::Output {
        Cons(input.lift_domain_function(), Nil)
    }
}
*/
