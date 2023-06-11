use t_funk::{
    closure::Closure,
    collection::set::{Get, Set},
};

use crate::Distance;

/// Evaluate two shapes in full, and blend them together using a binary function.
/// Primarily useful in single-domain contexts to avoid the double evaluation of PreBoolean.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct SmoothBoolean<F> {
    pub boolean: F,
    pub k: f32,
}

impl<F, A, B, C, FA, FB> Closure<(A, B, C, FA, FB)> for SmoothBoolean<F>
where
    C: Clone + Get<Distance<f32>> + Set<Distance<f32>>,
    FA: Closure<C, Output = C>,
    FB: Closure<C, Output = C>,
    F: Closure<(C, C), Output = bool>,
{
    type Output = C;

    fn call(self, (_, _, c, fa, fb): (A, B, C, FA, FB)) -> Self::Output {
        let ca = fa.call(c.clone());
        let cb = fb.call(c.clone());

        let Distance(da) = ca.clone().get();
        let Distance(db) = cb.clone().get();

        fn lerp(a: f32, b: f32, t: f32) -> f32 {
            a + (b - a) * t
        }

        let t = (0.5 + 0.5 * (db - da) / self.k).clamp(0.0, 1.0);
        let d = lerp(db, da, t) - self.k * t * (1.0 - t);

        if self.boolean.call((ca.clone(), cb.clone())) {
            ca.set(Distance(d))
        } else {
            cb.set(Distance(d))
        }
    }
}
