use std::marker::PhantomData;

use t_funk::{
    collection::map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
    collection::set::{Get as GetS, Insert as InsertS, InsertT as InsertST},
    function::Function,
    macros::{
        phantom::{PhantomClone, PhantomCopy, PhantomDefault},
        Closure,
    },
};

/// Overwrite O with I
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct CopyContext<I, O>(PhantomData<(I, O)>);

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
