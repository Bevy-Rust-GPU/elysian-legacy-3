use std::marker::PhantomData;

use t_funk::{
    closure::Closure,
    collection::set::{Get, Set},
};

use crate::{Distance, LiftEvaluate, LiftEvaluateT};

/// Use inclusion in shape A as a predicate for evaluating shape B
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounding<F>(pub F);

/// Evaluate distance of A, return infinity if outside, evaluate inner shape if inside
impl<F, A, B, C, D> Closure<(A, B, C, PhantomData<(Distance<f32>, D)>)> for Bounding<F>
where
    A: Clone + LiftEvaluate<(Distance<f32>, ())>,
    B: Clone + LiftEvaluate<(Distance<f32>, D)>,
    LiftEvaluateT<A, (Distance<f32>, ())>: Closure<C, Output = C>,
    LiftEvaluateT<B, (Distance<f32>, D)>: Closure<C, Output = C>,
    C: Default + Clone + Get<Distance<f32>> + Set<Distance<f32>>,
    F: Closure<(Distance<f32>, Distance<f32>), Output = bool>,
{
    type Output = C;

    fn call(self, (a, b, c, _): (A, B, C, PhantomData<(Distance<f32>, D)>)) -> Self::Output {
        let da = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(a.clone()).call(c.clone());

        if self.0.call((da.clone().get(), Default::default())) {
            LiftEvaluate::<(Distance<f32>, D)>::lift_evaluate(b).call(c)
        } else {
            C::default().set(Distance(f32::INFINITY))
        }
    }
}
