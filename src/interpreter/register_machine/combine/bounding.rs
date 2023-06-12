use t_funk::{closure::Closure, collection::set::Insert};

use crate::Distance;

/// Use inclusion in shape A as a predicate for evaluating shape B
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounding<F>(pub F);

/// Evaluate distance of A, return infinity if outside, evaluate inner shape if inside
impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for Bounding<F>
where
    A: Closure<C, Output = C>,
    B: Closure<C, Output = C>,
    C: Default + Clone + Insert<Distance<f32>, Insert = C>,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
{
    type Output = C;

    fn call(self, (a, _, c, _, fb): (A, B, C, FA, FB)) -> Self::Output {
        let da = a.call(c.clone());

        if self.0.call((da, Default::default())) {
            fb.call(c)
        } else {
            C::default().insert(Distance(f32::INFINITY))
        }
    }
}
