use t_funk::{
    closure::{Closure, ComposeLF, Curry2, Curry2B, OutputT},
    function::Id,
    macros::{functions, types},
    typeclass::{
        foldable::{Foldr, FoldrT},
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT},
    },
};

use crate::{ExpandAliasF, LiftAdtF, LiftEvaluateF, LiftParamF};

/// Given a list of domains, a shape, and a context,
/// evaluate the shape's domain functions and produce an updated context
#[types]
#[functions]
pub trait EvaluateImpl<D, C> {
    type EvaluateImpl;

    fn evaluate_impl(self, input: C) -> Self::EvaluateImpl;
}

impl<T, D, C> EvaluateImpl<D, C> for T
where
    C: Clone,
    T: Fmap<LiftAdtF>,
    FmapT<T, LiftAdtF>: Fmap<Curry2B<LiftParamF, C>>,
    FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>: Chain<ExpandAliasF<D>>,
    ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF<D>>:
        Fmap<LiftEvaluateF<D>>,
    FmapT<
        ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF<D>>,
        LiftEvaluateF<D>,
    >: Foldr<ComposeLF, Id>,
    FoldrT<
        FmapT<
            ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF<D>>,
            LiftEvaluateF<D>,
        >,
        ComposeLF,
        Id,
    >: Closure<C>,
{
    type EvaluateImpl = OutputT<
        FoldrT<
            FmapT<
                ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF<D>>,
                LiftEvaluateF<D>,
            >,
            ComposeLF,
            Id,
        >,
        C,
    >;

    fn evaluate_impl(self, input: C) -> Self::EvaluateImpl {
        self.fmap(LiftAdtF)
            .fmap(LiftParamF.suffix2(input.clone()))
            .chain(ExpandAliasF::<D>::default())
            .fmap(LiftEvaluateF::<D>::default())
            .foldr(ComposeLF, Id)
            .call(input)
    }
}

pub trait Evaluate<C>: Sized {
    fn evaluate<D>(self, input: C) -> EvaluateImplT<Self, D, C>
    where
        Self: EvaluateImpl<D, C>,
    {
        EvaluateImpl::<D, C>::evaluate_impl(self, input)
    }
}

impl<T, D> Evaluate<D> for T where T: Sized {}
