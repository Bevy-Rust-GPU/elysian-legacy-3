use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::set::{Get, GetT},
};

use crate::{Dist, LiftEvaluate, LiftEvaluateT};

// Given two evaluated contexts, pick one using the output of a (C, C) -> bool function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BooleanCombine<F, T>(pub F, pub PhantomData<T>);

impl<F, T, C> Closure<(C, C)> for BooleanCombine<F, T>
where
    C: Clone + Get<T>,
    F: Closure<(GetT<C, T>, GetT<C, T>), Output = bool>,
{
    type Output = C;

    fn call(self, (ca, cb): (C, C)) -> Self::Output {
        let ta = ca.clone().get();
        let tb = cb.clone().get();

        if self.0.call((ta, tb)) {
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
