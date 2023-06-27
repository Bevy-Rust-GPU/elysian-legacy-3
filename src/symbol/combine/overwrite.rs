use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
    collection::set::{Get as GetS, Insert as InsertS, InsertT as InsertST},
    function::Function,
    macros::{
        phantom::{PhantomClone, PhantomCopy, PhantomDefault},
        Closure,
    },
};

use crate::{EvaluateFunction, LiftAdt, Run};

/// Overwrite O with I
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct CopyContext<I, O>(PhantomData<(I, O)>);

impl<I, O> LiftAdt for CopyContext<I, O> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<I, O, D> EvaluateFunction<D> for CopyContext<I, O> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<I, O, C> Function<C> for CopyContext<I, O>
where
    C: Clone + GetM<I> + InsertM<O, GetMT<C, I>>,
{
    type Output = InsertMT<C, O, GetMT<C, I>>;

    fn call(ctx: C) -> Self::Output {
        ctx.clone().insert(ctx.get())
    }
}

/// Overwrite O with I
#[derive(
    Debug, PhantomDefault, PhantomCopy, PhantomClone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure,
)]
pub struct CopyProperty<T, I, O>(PhantomData<(T, I, O)>);

impl<T, I, O> LiftAdt for CopyProperty<T, I, O> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<T, I, O, D> EvaluateFunction<D> for CopyProperty<T, I, O> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<T, I, O, C> Function<C> for CopyProperty<T, I, O>
where
    C: Clone + GetM<I> + GetM<O> + InsertM<O, InsertST<GetMT<C, O>, T>>,
    GetMT<C, I>: GetS<T>,
    GetMT<C, O>: InsertS<T>,
{
    type Output = InsertMT<C, O, InsertST<GetMT<C, O>, T>>;

    fn call(ctx: C) -> Self::Output {
        let c = GetM::<I>::get(ctx.clone());
        let t = GetS::<T>::get(c);

        let c = GetM::<O>::get(ctx.clone());
        let c = InsertS::<T>::insert(c, t);

        ctx.insert(c)
    }
}

/// Overwrite O with I
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapProperty<I, T, F>(pub F, pub PhantomData<(T, I)>);

impl<I, T, F> LiftAdt for MapProperty<I, T, F> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<I, T, F, D> EvaluateFunction<D> for MapProperty<I, T, F> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<I, T, F, C> Closure<C> for MapProperty<I, T, F>
where
    C: Clone + GetM<I> + InsertM<I, InsertST<GetMT<C, I>, OutputT<F, T>>>,
    GetMT<C, I>: Clone + GetS<T> + InsertS<OutputT<F, T>>,
    F: Closure<T>,
{
    type Output = InsertMT<C, I, InsertST<GetMT<C, I>, OutputT<F, T>>>;

    fn call(self, ctx: C) -> Self::Output {
        let c = GetM::<I>::get(ctx.clone());

        let t = GetS::<T>::get(c.clone());
        let t = self.0.call(t);

        let c = InsertS::<OutputT<F, T>>::insert(c, t);

        ctx.insert(c)
    }
}
