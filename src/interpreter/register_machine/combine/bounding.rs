use t_funk::{
    closure::Closure,
    collection::set::{Get, Set},
};

use crate::Distance;

/// Use inclusion in shape A as a predicate for evaluating shape B
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounding<F>(pub F);

/// Evaluate distance of A, return infinity if outside, evaluate inner shape if inside
impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for Bounding<F>
where
    A: Closure<C, Output = C>,
    B: Closure<C, Output = C>,
    C: Default + Clone + Get<Distance<f32>> + Set<Distance<f32>>,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
{
    type Output = C;

    fn call(self, (a, _, c, _, fb): (A, B, C, FA, FB)) -> Self::Output {
        let da = a.call(c.clone());

        if self.0.call((da.clone(), Default::default())) {
            fb.call(c)
        } else {
            C::default().set(Distance(f32::INFINITY))
        }
    }
}
