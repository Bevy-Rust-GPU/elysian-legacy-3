use std::marker::PhantomData;

use t_funk::{
    closure::Closure,
    collection::map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
    collection::set::{Insert as InsertS, InsertT as InsertST},
};

use crate::{LiftAdt, LiftEvaluate};

/// Overwrite O with the provided property T
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InsertProperty<T, O>(pub T, pub PhantomData<O>);

impl<T, O> LiftAdt for InsertProperty<T, O> {
    type LiftAdt = Self;

    fn lift_adt(self) -> Self::LiftAdt {
        self
    }
}

impl<T, O, D> LiftEvaluate<D> for InsertProperty<T, O> {
    type LiftEvaluate = (Self,);

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        (self,)
    }
}

impl<T, O, C> Closure<C> for InsertProperty<T, O>
where
    C: Clone + GetM<O> + InsertM<O, InsertST<GetMT<C, O>, T>>,
    GetMT<C, O>: InsertS<T>,
{
    type Output = InsertMT<C, O, InsertST<GetMT<C, O>, T>>;

    fn call(self, ctx: C) -> Self::Output {
        let c = GetM::<O>::get(ctx.clone());
        let c = InsertS::<T>::insert(c, self.0);

        ctx.insert(c)
    }
}
