use t_funk::{closure::{Closure, OutputT}, macros::{types, functions}};

use crate::{LiftCombine, LiftCombineT, LiftEvaluate, LiftEvaluateT, LiftParam, LiftParamT};

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
    LiftParamT<T, C>: LiftCombine,
    LiftCombineT<LiftParamT<T, C>>: LiftEvaluate<D>,
    LiftEvaluateT<LiftCombineT<LiftParamT<T, C>>, D>: Closure<C>,
{
    type Evaluate = OutputT<LiftEvaluateT<LiftCombineT<LiftParamT<T, C>>, D>, C>;

    fn evaluate(self, input: C) -> Self::Evaluate {
        self.lift_param(input.clone())
            .lift_combine()
            .lift_evaluate()
            .call(input)
    }
}
