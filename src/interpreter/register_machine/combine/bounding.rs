use t_funk::{
    closure::{Closure, OutputT},
    collection::set::Insert,
};

use crate::Distance;

/// Use inclusion in shape A as a predicate for evaluating shape B
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounding<F>(pub F);

/// Evaluate distance of A, return infinity if outside, evaluate inner shape if inside
impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for Bounding<F>
where
    A: Closure<C>,
    C: Clone,
    OutputT<A, C>: Default + Clone + Insert<Distance<f32>, Insert = OutputT<A, C>>,
    FB: Closure<C, Output = OutputT<A, C>>,
    F: Closure<(OutputT<A, C>, OutputT<A, C>), Output = bool>,
{
    type Output = OutputT<A, C>;

    fn call(self, (a, _, c, _, fb): (A, B, C, FA, FB)) -> Self::Output {
        let ca = a.call(c.clone());
        let cb: OutputT<A, C> = Default::default();

        if self.0.call((ca, cb.clone())) {
            fb.call(c)
        } else {
            cb.insert(Distance(f32::INFINITY))
        }
    }
}
