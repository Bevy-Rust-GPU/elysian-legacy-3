use t_funk::{
    macros::{functions, types},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT},
    },
};

use crate::{Alias, Combine, LiftAdtF, Run};

#[functions]
#[types]
pub trait ExpandAlias<D> {
    type ExpandAlias;

    fn expand_alias(self) -> Self::ExpandAlias;
}

impl<A, D> ExpandAlias<D> for Run<A> {
    type ExpandAlias = (Self,);

    fn expand_alias(self) -> Self::ExpandAlias {
        (self,)
    }
}

impl<A, D> ExpandAlias<D> for Alias<A>
where
    A: ExpandAlias<D>,
    ExpandAliasT<A, D>: Fmap<LiftAdtF>,
{
    type ExpandAlias = FmapT<ExpandAliasT<A, D>, LiftAdtF>;

    fn expand_alias(self) -> Self::ExpandAlias {
        self.0.expand_alias().fmap(LiftAdtF)
    }
}

impl<A, B, C, D> ExpandAlias<D> for Combine<A, B, C>
where
    A: Chain<ExpandAliasF<D>>,
    B: Chain<ExpandAliasF<D>>,
    C: Chain<ExpandAliasF<D>>,
{
    type ExpandAlias = (
        Combine<ChainT<A, ExpandAliasF<D>>, ChainT<B, ExpandAliasF<D>>, ChainT<C, ExpandAliasF<D>>>,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        (Combine(
            self.0.chain(ExpandAliasF::<D>::default()),
            self.1.chain(ExpandAliasF::<D>::default()),
            self.2.chain(ExpandAliasF::<D>::default()),
        ),)
    }
}
