use t_funk::{
    closure::{Closure, OutputT},
    macros::{functions, types},
};

use crate::{LiftEvaluate, LiftEvaluateT, LiftParam, LiftParamT};

/// Given a list of domains, a shape, and a context,
/// evaluate the shape's domain functions and produce an updated context
#[types]
#[functions]
pub trait Evaluate<D, C> {
    type Evaluate;

    fn evaluate(self, input: C) -> Self::Evaluate;
}

impl<T, D, C> Evaluate<D, C> for T
where
    C: Clone,
    T: LiftParam<C>,
    LiftParamT<T, C>: LiftEvaluate<D>,
    LiftEvaluateT<LiftParamT<T, C>, D>: Closure<C>,
{
    type Evaluate = OutputT<LiftEvaluateT<LiftParamT<T, C>, D>, C>;

    fn evaluate(self, input: C) -> Self::Evaluate {
        self.lift_param(input.clone()).lift_evaluate().call(input)
    }
}
