use std::marker::PhantomData;

use t_funk::{closure::Closure, collection::set::Get};

use crate::{DistanceF32, LiftEvaluate, LiftEvaluateT, Pair};

/// Combine two shapes using a boolean function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean<F>(pub F);

/// Simplified impl for distance-only domain
impl<F, A, B, C> Closure<(A, B, C, PhantomData<(DistanceF32, ())>)> for Boolean<F>
where
    A: Clone + LiftEvaluate<(DistanceF32, ())>,
    B: Clone + LiftEvaluate<(DistanceF32, ())>,
    LiftEvaluateT<A, (DistanceF32, ())>: Closure<C, Output = C>,
    LiftEvaluateT<B, (DistanceF32, ())>: Closure<C, Output = C>,
    C: Clone + Get<DistanceF32>,
    F: Closure<(DistanceF32, DistanceF32), Output = bool>,
{
    type Output = C;

    fn call(self, (a, b, c, _): (A, B, C, PhantomData<(DistanceF32, ())>)) -> Self::Output {
        let da = LiftEvaluate::<(DistanceF32, ())>::lift_evaluate(a.clone()).call(c.clone());
        let db = LiftEvaluate::<(DistanceF32, ())>::lift_evaluate(b.clone()).call(c.clone());

        if self.0.call((da.clone().get(), db.clone().get())) {
            da
        } else {
            db
        }
    }
}

/// Evaluate distance, run boolean test, then evaluate the full domain of the resulting side
impl<F, A, B, C, D> Closure<(A, B, C, PhantomData<(DistanceF32, D)>)> for Boolean<F>
where
    D: Pair,
    A: Clone + LiftEvaluate<(DistanceF32, ())> + LiftEvaluate<(DistanceF32, D)>,
    B: Clone + LiftEvaluate<(DistanceF32, ())> + LiftEvaluate<(DistanceF32, D)>,
    LiftEvaluateT<A, (DistanceF32, ())>: Closure<C, Output = C>,
    LiftEvaluateT<B, (DistanceF32, ())>: Closure<C, Output = C>,
    LiftEvaluateT<A, (DistanceF32, D)>: Closure<C, Output = C>,
    LiftEvaluateT<B, (DistanceF32, D)>: Closure<C, Output = C>,
    C: Clone + Get<DistanceF32>,
    F: Closure<(DistanceF32, DistanceF32), Output = bool>,
{
    type Output = C;

    fn call(self, (a, b, c, _): (A, B, C, PhantomData<(DistanceF32, D)>)) -> Self::Output {
        let da = LiftEvaluate::<(DistanceF32, ())>::lift_evaluate(a.clone()).call(c.clone());
        let db = LiftEvaluate::<(DistanceF32, ())>::lift_evaluate(b.clone()).call(c.clone());

        if self.0.call((da.clone().get(), db.clone().get())) {
            LiftEvaluate::<(DistanceF32, D)>::lift_evaluate(a).call(c)
        } else {
            LiftEvaluate::<(DistanceF32, D)>::lift_evaluate(b).call(c)
        }
    }
}
