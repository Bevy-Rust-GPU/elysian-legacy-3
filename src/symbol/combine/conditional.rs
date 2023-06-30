use core::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::{
        map::{Get as GetM, GetT as GetMT},
        set::Get as GetS,
    },
    typeclass::monad::Identity,
};

use crate::{ContextA, ContextB, EvaluateFunction, IntoMonad, LiftAdt, Run};

// Call FA or FB depending on the output of a binary function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryConditional<T, F, FA, FB>(pub F, pub FA, pub FB, pub PhantomData<T>);

impl<T, F, FA, FB> IntoMonad for BinaryConditional<T, F, FA, FB> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T, F, FA, FB> LiftAdt for BinaryConditional<T, F, FA, FB> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, F, FA, FB, D> EvaluateFunction<D> for BinaryConditional<T, F, FA, FB> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<T, F, FA, FB, C> Closure<C> for BinaryConditional<T, F, FA, FB>
where
    C: Clone + GetM<ContextA> + GetM<ContextB>,
    GetMT<C, ContextA>: GetS<T>,
    GetMT<C, ContextB>: GetS<T>,
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

// Call FA or FB depending on the output of a binary function
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnaryConditional<C, T, F, FA, FB>(pub F, pub FA, pub FB, pub PhantomData<(C, T)>);

impl<C, T, F, FA, FB> LiftAdt for UnaryConditional<C, T, F, FA, FB> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<C, T, F, FA, FB, D> EvaluateFunction<D> for UnaryConditional<C, T, F, FA, FB> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<C, T, F, FA, FB, D> Closure<D> for UnaryConditional<C, T, F, FA, FB>
where
    D: Clone + GetM<C>,
    GetMT<D, C>: GetS<T>,
    F: Closure<T, Output = bool>,
    FA: Closure<D>,
    FB: Closure<D, Output = OutputT<FA, D>>,
{
    type Output = OutputT<FA, D>;

    fn call(self, ctx: D) -> Self::Output {
        let ca = GetM::<C>::get(ctx.clone());

        let ta = ca.get();

        if self.0.call(ta) {
            self.1.call(ctx)
        } else {
            self.2.call(ctx)
        }
    }
}
