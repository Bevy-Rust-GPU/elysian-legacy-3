use core::marker::PhantomData;

use crate::{
    Dist, Distance, EvaluateImpl, EvaluateImplT, ExpandAlias, ExpandAliasF, Gradient, IntoMonad,
    LiftAdt, LiftAdtF, LiftEvaluate, LiftParam, LiftParamF, Pair, Position,
};

use crate::glam::Vec2;
use t_funk::{
    closure::{Closure, Curry2, Curry2B},
    collection::set::{Get, Insert, InsertT},
    typeclass::{
        functor::{Fmap, FmapT},
        monad::{Chain, ChainT, Identity},
    },
};

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Default, Copy, Clone, PartialEq)]
pub struct Reflect<T>(pub Vec2, pub T);

impl<T> IntoMonad for Reflect<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Reflect<T>
where
    T: Fmap<LiftAdtF>,
{
    type LiftAdt = Reflect<FmapT<T, LiftAdtF>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Reflect(self.0, self.1.fmap(LiftAdtF))
    }
}

impl<T, D> ExpandAlias<D> for Reflect<T>
where
    T: Chain<ExpandAliasF<D>>,
{
    type ExpandAlias = (Reflect<ChainT<T, ExpandAliasF<D>>>,);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Reflect(self.0, self.1.chain(ExpandAliasF::<D>::default())),)
    }
}

impl<T, C> LiftParam<C> for Reflect<T>
where
    T: Fmap<Curry2B<LiftParamF, C>>,
{
    type LiftParam = Reflect<FmapT<T, Curry2B<LiftParamF, C>>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Reflect(self.0, self.1.fmap(LiftParamF.suffix2(input)))
    }
}

impl<T, D> LiftEvaluate<D> for Reflect<T> {
    type LiftEvaluate = EvaluateReflect<T, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        EvaluateReflect(self.0, self.1, PhantomData::<D>)
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Default, Copy, Clone, PartialEq)]
pub struct EvaluateReflect<T, D>(Vec2, T, PhantomData<D>);

impl<T, C> Closure<C> for EvaluateReflect<T, Dist<f32>>
where
    C: Clone + Get<Position<Vec2>> + Insert<Position<Vec2>, Insert = C>,
    T: EvaluateImpl<Dist<f32>, C>,
{
    type Output = EvaluateImplT<T, Dist<f32>, C>;

    fn call(self, input: C) -> Self::Output {
        let n = self.0;
        let Position(p) = Get::<Position<Vec2>>::get(input.clone());
        let d = n.dot(p);
        let o = p - 2.0 * d * n;

        let pr = if d >= 0.0 { p } else { o };

        let input = Insert::<Position<Vec2>>::insert(input, Position(pr));

        EvaluateImpl::<Dist<f32>, C>::evaluate_impl(self.1, input)
    }
}

impl<T, D, C> Closure<C> for EvaluateReflect<T, (Distance<f32>, D)>
where
    D: Pair,
    C: Clone + Get<Position<Vec2>> + Insert<Position<Vec2>, Insert = C>,
    T: EvaluateImpl<(Distance<f32>, D), C>,
    EvaluateImplT<T, (Distance<f32>, D), C>: Clone + Get<Gradient<Vec2>> + Insert<Gradient<Vec2>>,
{
    type Output = InsertT<EvaluateImplT<T, (Distance<f32>, D), C>, Gradient<Vec2>>;

    fn call(self, input: C) -> Self::Output {
        let n = self.0;
        let Position(p) = Get::<Position<Vec2>>::get(input.clone());
        let d = n.dot(p);
        let o = p - 2.0 * d * n;

        let pr = if d >= 0.0 { p } else { o };

        let input = Insert::<Position<Vec2>>::insert(input, Position(pr));

        let input = EvaluateImpl::<(Distance<f32>, D), C>::evaluate_impl(self.1, input);

        let Gradient(g) = Get::<Gradient<Vec2>>::get(input.clone());
        let g = if n.dot(p) >= 0.0 {
            g
        } else {
            g - 2.0 * n.dot(g) * n
        };

        input.insert(Gradient(g))
    }
}
