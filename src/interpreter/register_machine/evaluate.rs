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
pub trait Evaluate<D, C> {
    type Evaluate;

    fn evaluate(self, input: C) -> Self::Evaluate;
}

impl<T, D, C> Evaluate<D, C> for T
where
    C: Clone,
    T: Fmap<LiftAdtF>,
    FmapT<T, LiftAdtF>: Fmap<Curry2B<LiftParamF, C>>,
    FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>: Chain<ExpandAliasF>,
    ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF>: Fmap<LiftEvaluateF<D>>,
    FmapT<
        ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF>,
        LiftEvaluateF<D>,
    >: Foldr<ComposeLF, Id>,
    FoldrT<
        FmapT<
            ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF>,
            LiftEvaluateF<D>,
        >,
        ComposeLF,
        Id,
    >: Closure<C>,
{
    type Evaluate = OutputT<
        FoldrT<
            FmapT<
                ChainT<FmapT<FmapT<T, LiftAdtF>, Curry2B<LiftParamF, C>>, ExpandAliasF>,
                LiftEvaluateF<D>,
            >,
            ComposeLF,
            Id,
        >,
        C,
    >;

    fn evaluate(self, input: C) -> Self::Evaluate {
        self.fmap(LiftAdtF)
            .fmap(LiftParamF.suffix2(input.clone()))
            .chain(ExpandAliasF)
            .fmap(LiftEvaluateF::<D>::default())
            .foldr(ComposeLF, Id)
            .call(input)
    }
}
