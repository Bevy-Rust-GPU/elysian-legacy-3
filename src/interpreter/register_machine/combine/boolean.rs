use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::{
        map::{Get as GetM, GetT as GetMT},
        set::Get as GetS,
    },
};

use crate::{ContextA, ContextB};

// Call FA or FB depending on the output of a binary function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BooleanConditional<F, FA, FB, T>(pub F, pub FA, pub FB, pub PhantomData<T>);

impl<F, FA, FB, T, C> Closure<C> for BooleanConditional<F, FA, FB, T>
where
    C: Clone + GetM<ContextA> + GetM<ContextB, Get = GetMT<C, ContextA>>,
    GetMT<C, ContextA>: GetS<T>,
    F: Closure<(T, T), Output = bool>,
    FA: Closure<C>,
    FB: Closure<C, Output = OutputT<FA, C>>,
{
    type Output = OutputT<FA, C>;

    fn call(self, ctx: C) -> Self::Output {
        let ca = GetM::<ContextA>::get(ctx.clone());
        let cb = GetM::<ContextB>::get(ctx.clone());

        let ta = ca.get();
        let tb = cb.get();

        if self.0.call((ta, tb)) {
            self.1.call(ctx)
        } else {
            self.2.call(ctx)
        }
    }
}
