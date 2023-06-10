use std::marker::PhantomData;

use t_funk::{closure::Closure, collection::set::Get};

use crate::{Distance, LiftEvaluate, LiftEvaluateT, Pair};

/// Combine two shapes using a boolean function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean<F>(pub F);

/*
/// Simplified impl for distance-only domain
impl<F, A, B, C> Closure<(A, B, C, PhantomData<(Distance<f32>, ())>)> for Boolean<F>
where
    A: LiftEvaluate<(Distance<f32>, ())>,
    B: LiftEvaluate<(Distance<f32>, ())>,
    LiftEvaluateT<A, (Distance<f32>, ())>: Closure<C, Output = C>,
    LiftEvaluateT<B, (Distance<f32>, ())>: Closure<C, Output = C>,
    C: Clone + Get<Distance<f32>>,
    F: Closure<(Distance<f32>, Distance<f32>), Output = bool>,
{
    type Output = C;

    fn call(self, (a, b, c, _): (A, B, C, PhantomData<(Distance<f32>, ())>)) -> Self::Output {
        let da = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(a).call(c.clone());
        let db = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(b).call(c.clone());

        if self.0.call((da.clone().get(), db.clone().get())) {
            da
        } else {
            db
        }
    }
}

/// Evaluate distance, run boolean test, then evaluate the full domain of the resulting side
impl<F, A, B, C, D> Closure<(A, B, C, PhantomData<(Distance<f32>, D)>)> for Boolean<F>
where
    D: Pair,
    A: Clone + LiftEvaluate<(Distance<f32>, ())> + LiftEvaluate<(Distance<f32>, D)>,
    B: Clone + LiftEvaluate<(Distance<f32>, ())> + LiftEvaluate<(Distance<f32>, D)>,
    LiftEvaluateT<A, (Distance<f32>, ())>: Closure<C, Output = C>,
    LiftEvaluateT<B, (Distance<f32>, ())>: Closure<C, Output = C>,
    LiftEvaluateT<A, (Distance<f32>, D)>: Closure<C, Output = C>,
    LiftEvaluateT<B, (Distance<f32>, D)>: Closure<C, Output = C>,
    C: Clone + Get<Distance<f32>>,
    F: Closure<(Distance<f32>, Distance<f32>), Output = bool>,
{
    type Output = C;

    fn call(self, (a, b, c, _): (A, B, C, PhantomData<(Distance<f32>, D)>)) -> Self::Output {
        let da = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(a.clone()).call(c.clone());
        let db = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(b.clone()).call(c.clone());

        if self.0.call((da.clone().get(), db.clone().get())) {
            LiftEvaluate::<(Distance<f32>, D)>::lift_evaluate(a).call(c)
        } else {
            LiftEvaluate::<(Distance<f32>, D)>::lift_evaluate(b).call(c)
        }
    }
}
*/

/// Evaluate distance, run boolean test, then evaluate the full domain of the resulting side
impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for Boolean<F>
where
    C: Clone + Get<Distance<f32>>,
    A: Closure<C, Output = C>,
    B: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
{
    type Output = C;

    fn call(self, (a, b, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let da = a.call(c.clone());
        let db = b.call(c.clone());

        if self.0.call((da, db)) {
            fa.call(c)
        } else {
            fb.call(c)
        }
    }
}
