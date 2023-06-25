use std::marker::PhantomData;

use t_funk::{
    closure::{CallF, Closure, Compose, ComposeLF, OutputT},
    collection::{
        map::{Get, GetT},
        set::{DropF, LiftContext, LiftContextT},
    },
    function::Id,
    macros::{functions, types},
    typeclass::{
        arrow::{Fanout, FanoutT},
        category::ComposeLT,
        foldable::{Foldr, FoldrT},
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT},
    },
};

use crate::{
    Combine, CombineContext, ContextOut, Domains, EvaluateFunction, EvaluateInputs, FunctionT,
    InputsT, LiftDomains, Modify, MovesT, Run, LiftDomainsT,
};

#[functions]
#[types]
pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

impl<A, D> LiftEvaluate<D> for Run<A>
where
    A: EvaluateFunction<D>,
{
    type LiftEvaluate = FunctionT<A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.evaluate_function()
    }
}

impl<A, D> LiftEvaluate<D> for Modify<A>
where
    A: EvaluateFunction<D> + EvaluateInputs<D>,
    FunctionT<A, D>: LiftContext<InputsT<A, D>>,
    LiftContextT<FunctionT<A, D>, InputsT<A, D>>: Fanout<DropF<MovesT<A, D>>>,
{
    type LiftEvaluate = ComposeLT<
        FanoutT<LiftContextT<FunctionT<A, D>, InputsT<A, D>>, DropF<MovesT<A, D>>>,
        CallF,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0
            .evaluate_function()
            .lift_context()
            .fanout(DropF::<MovesT<A, D>>::default())
            .compose_l(CallF)
    }
}

impl<A, D> LiftEvaluate<D> for Domains<A>
where
    D: LiftDomains<A>,
{
    type LiftEvaluate = LiftDomainsT<D, A>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        D::lift_domains(self.0)
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
