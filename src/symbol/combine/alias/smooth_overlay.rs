use core::marker::PhantomData;

use t_funk::{
    closure::{Closure, Curry2B},
    function::Lt,
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

use crate::{
    Alias, BlendProperty, BlendPropertyDist, ContextA, ContextB, ContextOut, CopyContext, Distance,
    EvaluateBoth, ExpandAlias, ExpandAliasT, Inherited, IntoMonad, IntoTuple, IntoTupleT, Lerp,
    LerpT, LiftAdt, UnaryConditional,
};

use t_funk::macros::{functions, types};

use crate::Combine;

#[functions]
#[types]
pub trait MakeSmoothOverlay<T> {
    type SmoothOverlay;

    fn smooth_overlay(self, k: f32, rhs: T) -> Self::SmoothOverlay;
}

impl<T, U> MakeSmoothOverlay<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type SmoothOverlay = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<SmoothOverlay>>;

    fn smooth_overlay(self, k: f32, rhs: U) -> Self::SmoothOverlay {
        Combine(
            self.into_tuple(),
            rhs.into_tuple(),
            SmoothOverlay(k).into_tuple(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothOverlay(pub f32);

pub fn smooth_overlay() -> SmoothOverlay {
    SmoothOverlay(1.0)
}

impl SmoothOverlay {
    pub fn k(self, k: f32) -> Self {
        SmoothOverlay(k)
    }
}

impl<F> Fmap<F> for SmoothOverlay {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for SmoothOverlay {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for SmoothOverlay {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for SmoothOverlay
where
    EvaluateBoth<Inherited>: ExpandAlias<D>,
    ExpandAliasT<EvaluateBoth<Inherited>, D>: Mappend<(
        UnaryConditional<
            ContextB,
            Distance<f32>,
            Curry2B<Lt, Distance<f32>>,
            CopyContext<ContextB, ContextOut>,
            CopyContext<ContextA, ContextOut>,
        >,
        BlendProperty<PolynomialSmoothOverlay<Distance<f32>>, Distance<f32>>,
    )>,
{
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<Inherited>, D>,
        (
            UnaryConditional<
                ContextB,
                Distance<f32>,
                Curry2B<Lt, Distance<f32>>,
                CopyContext<ContextB, ContextOut>,
                CopyContext<ContextA, ContextOut>,
            >,
            BlendProperty<PolynomialSmoothOverlay<Distance<f32>>, Distance<f32>>,
        ),
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        EvaluateBoth::<Inherited>::default()
            .expand_alias()
            .mappend((
                UnaryConditional::<
                    ContextB,
                    Distance<f32>,
                    Curry2B<Lt, Distance<f32>>,
                    CopyContext<ContextB, ContextOut>,
                    CopyContext<ContextA, ContextOut>,
                >::default(),
                BlendProperty(
                    PolynomialSmoothOverlay(self.0, PhantomData::<Distance<f32>>),
                    PhantomData::<Distance<f32>>,
                ),
            ))
    }
}

pub fn polynomial_smooth_overlay<T>(k: f32) -> BlendPropertyDist<PolynomialSmoothOverlay<T>, T> {
    BlendPropertyDist(
        PolynomialSmoothOverlay(k, PhantomData::<T>),
        PhantomData::<T>,
    )
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct PolynomialSmoothOverlay<T>(pub f32, pub PhantomData<T>);

impl<T> Closure<(Distance<f32>, Distance<f32>)> for PolynomialSmoothOverlay<T>
where
    T: Lerp<T, f32>,
{
    type Output = Distance<f32>;

    fn call(self, (Distance(da), Distance(db)): (Distance<f32>, Distance<f32>)) -> Self::Output {
        let t = (0.5 + 0.5 * db / self.0).clamp(0.0, 1.0);
        let d = db.lerp(da, t);
        Distance(d)
    }
}

impl<T> Closure<(Distance<f32>, Distance<f32>, T, T)> for PolynomialSmoothOverlay<T>
where
    T: Lerp<T, f32>,
{
    type Output = LerpT<T, T, f32>;

    fn call(self, (_, Distance(db), pa, pb): (Distance<f32>, Distance<f32>, T, T)) -> Self::Output {
        let t = (0.5 + 0.5 * db / self.0).clamp(0.0, 1.0);
        pb.lerp(pa, t)
    }
}
