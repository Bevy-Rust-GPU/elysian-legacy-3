use t_funk::closure::Closure;

use crate::{Dist, LiftEvaluate, LiftEvaluateT};

/// Evaluate two shapes in full, then pick one based on the output of a binary function
/// Primarily useful in single-domain contexts to avoid the double evaluation of PreBoolean.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PostBoolean<F>(pub F);

impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for PostBoolean<F>
where
    C: Clone,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
{
    type Output = C;

    fn call(self, (_, _, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let da = fa.call(c.clone());
        let db = fb.call(c.clone());

        if self.0.call((da.clone(), db.clone())) {
            da
        } else {
            db
        }
    }
}

/// Evaluate the Distance domain of the given shapes,
/// and call the provided continuations based on the output of a binary function.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreBoolean<F>(pub F);

/// Evaluate distance, run boolean test, then evaluate the full domain of the resulting side
impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for PreBoolean<F>
where
    C: Clone,
    A: LiftEvaluate<Dist<f32>>,
    LiftEvaluateT<A, Dist<f32>>: Closure<C, Output = C>,
    B: LiftEvaluate<Dist<f32>>,
    LiftEvaluateT<B, Dist<f32>>: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
{
    type Output = C;

    fn call(self, (a, b, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let da = LiftEvaluate::<Dist<f32>>::lift_evaluate(a).call(c.clone());
        let db = LiftEvaluate::<Dist<f32>>::lift_evaluate(b).call(c.clone());

        if self.0.call((da, db)) {
            fa.call(c)
        } else {
            fb.call(c)
        }
    }
}
