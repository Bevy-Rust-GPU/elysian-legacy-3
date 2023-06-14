use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::set::{Get, GetT},
};

// Given two evaluated contexts, calculate a blending factor using a (C, C) -> T function,
// then blend them using a (C, C, T) -> C function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendCombine<TF, BF, T>(pub TF, pub BF, pub PhantomData<T>);

impl<TF, BF, T, C> Closure<(C, C)> for BlendCombine<TF, BF, T>
where
    C: Clone + Get<T>,
    TF: Closure<(GetT<C, T>, GetT<C, T>)>,
    BF: Closure<(C, C, OutputT<TF, (GetT<C, T>, GetT<C, T>)>)>,
{
    type Output = OutputT<BF, (C, C, OutputT<TF, (GetT<C, T>, GetT<C, T>)>)>;

    fn call(self, (ca, cb): (C, C)) -> Self::Output {
        let ta = ca.clone().get();
        let tb = cb.clone().get();

        let t = self.0.call((ta, tb));
        self.1.call((ca, cb, t))
    }
}
