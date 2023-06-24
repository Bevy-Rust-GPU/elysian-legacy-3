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
pub trait ExpandAlias {
    type ExpandAlias;

    fn expand_alias(self) -> Self::ExpandAlias;
}

impl<A> ExpandAlias for Run<A> {
    type ExpandAlias = (Self,);

    fn expand_alias(self) -> Self::ExpandAlias {
        (self,)
    }
}

impl<A> ExpandAlias for Alias<A>
where
    A: ExpandAlias,
    ExpandAliasT<A>: Fmap<LiftAdtF>,
{
    type ExpandAlias = FmapT<ExpandAliasT<A>, LiftAdtF>;

    fn expand_alias(self) -> Self::ExpandAlias {
        self.0.expand_alias().fmap(LiftAdtF)
    }
}

impl<A, B, C> ExpandAlias for Combine<A, B, C>
where
    A: Chain<ExpandAliasF>,
    B: Chain<ExpandAliasF>,
{
    type ExpandAlias = (Combine<ChainT<A, ExpandAliasF>, ChainT<B, ExpandAliasF>, C>,);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Combine(
            self.0.chain(ExpandAliasF),
            self.1.chain(ExpandAliasF),
            self.2,
        ),)
    }
}
