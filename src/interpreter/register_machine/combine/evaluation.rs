use t_funk::closure::{Closure, OutputT};

// Evaluate two domains in full, and pass the results to a continuation
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluateAndCombine<F>(pub F);

impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for EvaluateAndCombine<F>
where
    C: Clone,
    FA: Closure<C>,
    FB: Closure<C>,
    F: Closure<(OutputT<FA, C>, OutputT<FB, C>)>,
{
    type Output = OutputT<F, (OutputT<FA, C>, OutputT<FB, C>)>;

    fn call(self, (_, _, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let ca = fa.call(c.clone());
        let cb = fb.call(c);

        self.0.call((ca, cb))
    }
}

