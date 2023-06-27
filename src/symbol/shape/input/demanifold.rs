use std::marker::PhantomData;

use crate::{
    Distance, Evaluate, EvaluateT, ExpandAlias, ExpandAliasF, IntoMonad, LiftAdt, LiftAdtF,
    LiftEvaluate, LiftParam, LiftParamF, Position,
};

use glam::Vec2;
use t_funk::{
    closure::{Closure, Curry2, Curry2B},
    collection::set::{Get, Insert},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT, Identity},
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Demanifold<T>(pub Vec2, pub T);

impl<T> IntoMonad for Demanifold<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Demanifold<T>
where
    T: Fmap<LiftAdtF>,
{
    type LiftAdt = Demanifold<FmapT<T, LiftAdtF>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Demanifold(self.0, self.1.fmap(LiftAdtF))
    }
}

impl<T, D> ExpandAlias<D> for Demanifold<T>
where
    T: Chain<ExpandAliasF<D>>,
{
    type ExpandAlias = (Demanifold<ChainT<T, ExpandAliasF<D>>>,);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Demanifold(
            self.0,
            self.1.chain(ExpandAliasF::<D>::default()),
        ),)
    }
}

impl<T, C> LiftParam<C> for Demanifold<T>
where
    T: Fmap<Curry2B<LiftParamF, C>>,
{
    type LiftParam = Demanifold<FmapT<T, Curry2B<LiftParamF, C>>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Demanifold(self.0, self.1.fmap(LiftParamF.suffix2(input)))
    }
}

impl<T, D> LiftEvaluate<D> for Demanifold<T> {
    type LiftEvaluate = EvaluateDemanifold<T, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluateDemanifold(self.0, self.1, PhantomData::<D>)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EvaluateDemanifold<T, D>(Vec2, T, PhantomData<D>);

impl<T, D, C> Closure<C> for EvaluateDemanifold<T, D>
where
    C: Clone + Get<Position<Vec2>> + Insert<Position<Vec2>, Insert = C>,
    T: Evaluate<D, C>,
    EvaluateT<T, D, C>:
        Clone + Get<Distance<f32>> + Insert<Distance<f32>, Insert = EvaluateT<T, D, C>>,
{
    type Output = EvaluateT<T, D, C>;

    fn call(self, input: C) -> Self::Output {
        let n = self.0;
        let Position(p) = Get::<Position<Vec2>>::get(input.clone());
        let s = n.dot(p).signum();

        let input = Evaluate::<D, C>::evaluate(self.1, input);

        let Distance(d) = Get::<Distance<f32>>::get(input.clone());
        Insert::<Distance<f32>>::insert(input, Distance(d * s))
    }
}
