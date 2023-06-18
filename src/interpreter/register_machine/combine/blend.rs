use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, OutputT},
    collection::{
        hlist::{Cons, Nil},
        map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
        set::{Get as GetS, Insert as InsertS, InsertT as InsertST},
    },
};

use crate::{ContextA, ContextB, ContextOut, Distance, LiftEvaluate};

// Fetch a given property P from ContextA and ContextB,
// combine using a (P, P) -> P function, and write it to ContextOut
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct BlendProperty<F, T>(pub F, pub PhantomData<T>);

impl<F, T, D> LiftEvaluate<D> for BlendProperty<F, T> {
    type LiftEvaluate = Cons<Self, Nil>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Cons(self, Nil)
    }
}

impl<F, T, C> Closure<C> for BlendProperty<F, T>
where
    C: Clone
        + GetM<ContextA>
        + GetM<ContextB>
        + GetM<ContextOut>
        + InsertM<ContextOut, InsertST<GetMT<C, ContextOut>, OutputT<F, (T, T)>>>,
    GetMT<C, ContextA>: GetS<T>,
    GetMT<C, ContextB>: GetS<T>,
    GetMT<C, ContextOut>: InsertS<OutputT<F, (T, T)>>,
    F: Closure<(T, T)>,
{
    type Output = InsertMT<C, ContextOut, InsertST<GetMT<C, ContextOut>, OutputT<F, (T, T)>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_a = GetM::<ContextA>::get(ctx.clone());
        let context_b = GetM::<ContextB>::get(ctx.clone());
        let context_out = GetM::<ContextOut>::get(ctx.clone());

        let pa = context_a.get();
        let pb = context_b.get();

        let context_out = context_out.insert(self.0.call((pa, pb)));
        ctx.insert(context_out)
    }
}

// Fetch distance D and a given property P from ContextA and ContextB,
// combine using a (D, D, P, P) -> P function, and write it to ContextOut
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct BlendPropertyDist<F, T>(pub F, pub PhantomData<T>);

impl<F, T, D> LiftEvaluate<D> for BlendPropertyDist<F, T> {
    type LiftEvaluate = Cons<Self, Nil>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        Cons(self, Nil)
    }
}

impl<F, T, C> Closure<C> for BlendPropertyDist<F, T>
where
    C: Clone
        + GetM<ContextA>
        + GetM<ContextB>
        + GetM<ContextOut>
        + InsertM<
            ContextOut,
            InsertST<GetMT<C, ContextOut>, OutputT<F, (Distance<f32>, Distance<f32>, T, T)>>,
        >,
    GetMT<C, ContextA>: Clone + GetS<Distance<f32>> + GetS<T>,
    GetMT<C, ContextB>: Clone + GetS<Distance<f32>> + GetS<T>,
    F: Closure<(Distance<f32>, Distance<f32>, T, T)>,
    GetMT<C, ContextOut>: InsertS<OutputT<F, (Distance<f32>, Distance<f32>, T, T)>>,
{
    type Output = InsertMT<
        C,
        ContextOut,
        InsertST<GetMT<C, ContextOut>, OutputT<F, (Distance<f32>, Distance<f32>, T, T)>>,
    >;

    fn call(self, ctx: C) -> Self::Output {
        let context_a = GetM::<ContextA>::get(ctx.clone());
        let context_b = GetM::<ContextB>::get(ctx.clone());
        let context_out = GetM::<ContextOut>::get(ctx.clone());

        let da = GetS::<Distance<f32>>::get(context_a.clone());
        let db = GetS::<Distance<f32>>::get(context_b.clone());

        let pa = GetS::<T>::get(context_a);
        let pb = GetS::<T>::get(context_b);

        let context_out = context_out.insert(self.0.call((da, db, pa, pb)));
        ctx.insert(context_out)
    }
}
