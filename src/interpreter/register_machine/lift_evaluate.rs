use std::marker::PhantomData;

use t_funk::{
    closure::{Closure, ComposeLF, OutputT},
    collection::map::{Get, GetT},
    function::Id,
    macros::{functions, types},
    typeclass::{
        foldable::{Foldr, FoldrT},
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT},
    },
};

use crate::{
    Alias, Combine, CombineContext, ContextOut, Evaluable, EvaluableT, LiftEvaluable,
    LiftEvaluableT, Run,
};

#[functions]
#[types]
pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

impl<A, D> LiftEvaluate<D> for Run<A>
where
    A: Evaluable,
    EvaluableT<A>: LiftEvaluable<A, D>,
{
    type LiftEvaluate = LiftEvaluableT<EvaluableT<A>, A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluableT::<A>::lift_evaluable(self.0)
    }
}

impl<A, D> LiftEvaluate<D> for Alias<A>
where
    A: Evaluable,
    EvaluableT<A>: LiftEvaluable<A, D>,
{
    type LiftEvaluate = LiftEvaluableT<EvaluableT<A>, A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluableT::<A>::lift_evaluable(self.0)
    }
}

impl<A, B, F, D> LiftEvaluate<D> for Combine<A, B, F>
where
    F: Chain<LiftEvaluateF<D>>,
    ChainT<F, LiftEvaluateF<D>>: Foldr<ComposeLF, Id>,
{
    type LiftEvaluate =
        LiftEvaluateCombine<A, B, FoldrT<ChainT<F, LiftEvaluateF<D>>, ComposeLF, Id>, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftEvaluateCombine(
            self.0,
            self.1,
            self.2
                .chain(LiftEvaluateF::<D>::default())
                .foldr(ComposeLF::default(), Id),
            PhantomData,
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftEvaluateCombine<A, B, F, D>(pub A, pub B, pub F, pub PhantomData<D>);

impl<A, B, F, D, CI> Closure<CI> for LiftEvaluateCombine<A, B, F, D>
where
    A: Clone + Fmap<LiftEvaluateF<D>>,
    FmapT<A, LiftEvaluateF<D>>: Foldr<ComposeLF, Id>,
    B: Clone + Fmap<LiftEvaluateF<D>>,
    FmapT<B, LiftEvaluateF<D>>: Foldr<ComposeLF, Id>,
    F: Closure<
        CombineContext<
            A,
            B,
            CI,
            (),
            (),
            (),
            FoldrT<FmapT<A, LiftEvaluateF<D>>, ComposeLF, Id>,
            FoldrT<FmapT<B, LiftEvaluateF<D>>, ComposeLF, Id>,
        >,
    >,
    OutputT<
        F,
        CombineContext<
            A,
            B,
            CI,
            (),
            (),
            (),
            FoldrT<FmapT<A, LiftEvaluateF<D>>, ComposeLF, Id>,
            FoldrT<FmapT<B, LiftEvaluateF<D>>, ComposeLF, Id>,
        >,
    >: Get<ContextOut>,
{
    type Output = GetT<
        OutputT<
            F,
            CombineContext<
                A,
                B,
                CI,
                (),
                (),
                (),
                FoldrT<FmapT<A, LiftEvaluateF<D>>, ComposeLF, Id>,
                FoldrT<FmapT<B, LiftEvaluateF<D>>, ComposeLF, Id>,
            >,
        >,
        ContextOut,
    >;

    fn call(self, input: CI) -> Self::Output {
        self.2
            .call(CombineContext {
                shape_a: self.0.clone(),
                shape_b: self.1.clone(),
                context_in: input,
                context_a: (),
                context_b: (),
                context_out: (),
                inherited_a: self
                    .0
                    .fmap(LiftEvaluateF::<D>::default())
                    .foldr(ComposeLF, Id),
                inherited_b: self
                    .1
                    .fmap(LiftEvaluateF::<D>::default())
                    .foldr(ComposeLF, Id),
            })
            .get()
    }
}
