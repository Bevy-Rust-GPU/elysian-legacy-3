use t_funk::closure::{Closure, OutputT};

use crate::{Dist, LiftEvaluate, LiftEvaluateT};

/// Evaluate two shapes in full, then pick one based on the output of a binary function
/// Primarily useful in single-domain contexts to avoid the double evaluation of PreBoolean.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PostBoolean<F>(pub F);

impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for PostBoolean<F>
where
    C: Clone,
    FA: Closure<C>,
    FB: Closure<C, Output = OutputT<FA, C>>,
    OutputT<FA, C>: Clone,
    OutputT<FB, C>: Clone,
    F: Closure<(OutputT<FA, C>, OutputT<FB, C>), Output = bool>,
{
    type Output = OutputT<FA, C>;

    fn call(self, (_, _, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let ca = fa.call(c.clone());
        let cb = fb.call(c.clone());

        if self.0.call((ca.clone(), cb.clone())) {
            ca
        } else {
            cb
        }
    }
}

/// Evaluate the Distance domain of the given shapes,
/// and call one of the provided continuations based on the output of a binary function.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreBoolean<F>(pub F);

impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for PreBoolean<F>
where
    C: Clone,
    A: LiftEvaluate<Dist<f32>>,
    LiftEvaluateT<A, Dist<f32>>: Closure<C>,
    B: LiftEvaluate<Dist<f32>>,
    LiftEvaluateT<B, Dist<f32>>: Closure<C>,
    F: Closure<
        (
            OutputT<LiftEvaluateT<A, Dist<f32>>, C>,
            OutputT<LiftEvaluateT<B, Dist<f32>>, C>,
        ),
        Output = bool,
    >,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
{
    type Output = C;

    fn call(self, (a, b, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let ca = LiftEvaluate::<Dist<f32>>::lift_evaluate(a).call(c.clone());
        let cb = LiftEvaluate::<Dist<f32>>::lift_evaluate(b).call(c.clone());

        if self.0.call((ca, cb)) {
            fa.call(c)
        } else {
            fb.call(c)
        }
    }
}
