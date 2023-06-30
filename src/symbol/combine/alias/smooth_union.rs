use core::marker::PhantomData;
use t_funk::{
    closure::Closure,
    function::Lt,
    macros::{functions, types},
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

use crate::{
    Alias, BlendProperty, Combine, Distance, EvaluateSelect, ExpandAlias, ExpandAliasT, Inherited,
    IntoMonad, IntoTuple, IntoTupleT, Lerp, LerpT, LiftAdt,
};

#[functions]
#[types]
pub trait MakeSmoothUnion<T> {
    type SmoothUnion;

    fn smooth_union(self, rhs: T, k: f32) -> Self::SmoothUnion;
}

impl<T, U> MakeSmoothUnion<U> for T
where
    T: IntoTuple,
    U: IntoTuple,
{
    type SmoothUnion = Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<SmoothUnion>>;

    fn smooth_union(self, rhs: U, k: f32) -> Self::SmoothUnion {
        Combine(
            self.into_tuple(),
            rhs.into_tuple(),
            SmoothUnion(k).into_tuple(),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothUnion(f32);

impl<F> Fmap<F> for SmoothUnion {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for SmoothUnion {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for SmoothUnion {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for SmoothUnion
where
    ExpandAliasT<EvaluateSelect<Inherited, Distance<f32>, Lt>, D>:
        Mappend<IntoTupleT<BlendProperty<PolynomialSmoothMin<Distance<f32>>, Distance<f32>>>>,
{
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateSelect<Inherited, Distance<f32>, Lt>, D>,
        IntoTupleT<BlendProperty<PolynomialSmoothMin<Distance<f32>>, Distance<f32>>>,
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        ExpandAlias::<D>::expand_alias(EvaluateSelect::<Inherited, Distance<f32>, Lt>::default())
            .mappend(
                BlendProperty(
                    PolynomialSmoothMin(self.0, PhantomData::<Distance<f32>>),
                    PhantomData::<Distance<f32>>,
                )
                .into_tuple(),
            )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct PolynomialSmoothMin<T>(pub f32, PhantomData<T>);

impl Closure<(Distance<f32>, Distance<f32>)> for PolynomialSmoothMin<Distance<f32>> {
    type Output = Distance<f32>;

    fn call(self, (Distance(da), Distance(db)): (Distance<f32>, Distance<f32>)) -> Self::Output {
        let t = (0.5 + 0.5 * (db - da) / self.0).clamp(0.0, 1.0);
        let d = db.lerp(da, t) - self.0 * t * (1.0 - t);
        Distance(d)
    }
}

impl<T> Closure<(Distance<f32>, Distance<f32>, T, T)> for PolynomialSmoothMin<T>
where
    T: Lerp<T, f32>,
{
    type Output = LerpT<T, T, f32>;

    fn call(
        self,
        (Distance(da), Distance(db), pa, pb): (Distance<f32>, Distance<f32>, T, T),
    ) -> Self::Output {
        let t = (0.5 + 0.5 * (db - da) / self.0).clamp(0.0, 1.0);
        let p = pb.lerp(pa, t);
        p
    }
}
