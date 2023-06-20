use glam::Vec2;
use std::marker::PhantomData;
use t_funk::{
    closure::Closure,
    function::Lt,
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
    typeclass::monad::Identity,
};

use crate::{
    BlendProperty, BlendPropertyDist, BooleanConditional, Combine, ContextA, ContextB, ContextOut,
    CopyContext, Distance, EvaluateSide, Gradient, Inherited, Left, LiftAdtF, LiftEvaluate, Right,
    Run, Then,
};

pub fn smooth_union() -> OpChain<LiftAdtF, SmoothUnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait SmoothUnion<T> {
    type SmoothUnion;

    fn smooth_union(self, rhs: T, k: f32) -> Self::SmoothUnion;
}

impl_adt! {
    impl<A, B, C, R> SmoothUnion<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type SmoothUnion = Combine<Self, R, Identity<SmoothUnionS>>;

        fn smooth_union(self, rhs: R, k: f32) -> Self::SmoothUnion {
            Combine(self, rhs, Identity(SmoothUnionS(k)))
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothUnionS(f32);

impl<D> LiftEvaluate<D> for SmoothUnionS {
    type LiftEvaluate = (
        EvaluateSide<Left, Inherited, ContextA>,
        EvaluateSide<Right, Inherited, ContextB>,
        BooleanConditional<
            Lt,
            CopyContext<ContextA, ContextOut>,
            CopyContext<ContextB, ContextOut>,
            Distance<f32>,
        >,
        BlendProperty<PolynomialSmoothMin<Distance<f32>>, Distance<f32>>,
        BlendPropertyDist<PolynomialSmoothMin<Gradient<Vec2>>, Gradient<Vec2>>,
    );

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        (
            EvaluateSide::<Left, Inherited, ContextA>::default(),
            EvaluateSide::<Right, Inherited, ContextB>::default(),
            BooleanConditional(
                Lt,
                CopyContext::default(),
                CopyContext::default(),
                PhantomData::<Distance<f32>>,
            ),
            BlendProperty(
                PolynomialSmoothMin(self.0, PhantomData::<Distance<f32>>),
                PhantomData::<Distance<f32>>,
            ),
            BlendPropertyDist(
                PolynomialSmoothMin(self.0, PhantomData::<Gradient<Vec2>>),
                PhantomData::<Gradient<Vec2>>,
            ),
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

#[types]
pub trait Lerp<B, T> {
    type Lerp;

    fn lerp(self, b: B, t: T) -> Self::Lerp;
}

impl<A1, B1, A2, B2, T> Lerp<(A2, B2), T> for (A1, B1)
where
    A1: Lerp<A2, T>,
    B1: Lerp<B2, T>,
    T: Clone,
{
    type Lerp = (LerpT<A1, A2, T>, LerpT<B1, B2, T>);

    fn lerp(self, (a2, b2): (A2, B2), t: T) -> Self::Lerp {
        let (a1, b1) = self;
        (a1.lerp(a2, t.clone()), b1.lerp(b2, t))
    }
}

impl<A1, B1, C1, A2, B2, C2, T> Lerp<(A2, B2, C2), T> for (A1, B1, C1)
where
    A1: Lerp<A2, T>,
    B1: Lerp<B2, T>,
    C1: Lerp<C2, T>,
    T: Clone,
{
    type Lerp = (LerpT<A1, A2, T>, LerpT<B1, B2, T>, LerpT<C1, C2, T>);

    fn lerp(self, (a2, b2, c2): (A2, B2, C2), t: T) -> Self::Lerp {
        let (a1, b1, c1) = self;
        (
            a1.lerp(a2, t.clone()),
            b1.lerp(b2, t.clone()),
            c1.lerp(c2, t),
        )
    }
}

impl Lerp<f32, f32> for f32 {
    type Lerp = Self;

    fn lerp(self, b: f32, t: f32) -> Self::Lerp {
        self + (b - self) * t
    }
}

impl Lerp<Distance<f32>, f32> for Distance<f32> {
    type Lerp = Self;

    fn lerp(self, b: Distance<f32>, t: f32) -> Self::Lerp {
        Distance(self.0.lerp(b.0, t))
    }
}

impl Lerp<Gradient<Vec2>, f32> for Gradient<Vec2> {
    type Lerp = Self;

    fn lerp(self, b: Gradient<Vec2>, t: f32) -> Self::Lerp {
        Gradient(self.0.lerp(b.0, t))
    }
}
