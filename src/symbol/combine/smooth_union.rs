use glam::Vec2;
use t_funk::{
    closure::Closure,
    collection::set::{Get, Insert, InsertT},
    macros::{functions, impl_adt, types},
    op_chain::OpChain,
};

use crate::{
    BlendCombine, Combine, Distance, EvaluateAndCombine, Gradient, LiftAdtF, LiftCombine, Run, Then,
};

pub fn smooth_union() -> OpChain<LiftAdtF, SmoothUnionF> {
    Default::default()
}

#[functions]
#[types]
pub trait SmoothUnion<T> {
    type SmoothUnion;

    fn smooth_union(self, rhs: T) -> Self::SmoothUnion;
}

impl_adt! {
    impl<A, B, C, R> SmoothUnion<R> for Run<A> | Then<A, B> | Combine<A, B, C> {
        type SmoothUnion = Combine<Self, R, SmoothUnionS>;

        fn smooth_union(self, rhs: R) -> Self::SmoothUnion {
            Combine(self, rhs, SmoothUnionS)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SmoothUnionS;

impl<D> LiftCombine<D> for SmoothUnionS {
    type LiftCombine =
        EvaluateAndCombine<BlendCombine<NormalizedDistance, NormalizedLerp, Distance<f32>>>;

    fn lift_combine(self) -> Self::LiftCombine {
        Default::default()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct NormalizedDistance(f32);

impl Default for NormalizedDistance {
    fn default() -> Self {
        Self(0.2)
    }
}

impl Closure<(Distance<f32>, Distance<f32>)> for NormalizedDistance {
    type Output = f32;

    fn call(self, (Distance(da), Distance(db)): (Distance<f32>, Distance<f32>)) -> Self::Output {
        (0.5 + 0.5 * (db - da) / self.0).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct NormalizedLerp(f32);

impl Default for NormalizedLerp {
    fn default() -> Self {
        Self(0.25)
    }
}

impl<C> Closure<(C, C, f32)> for NormalizedLerp
where
    C: Clone + Get<(Distance<f32>, Gradient<Vec2>)> + Insert<(Distance<f32>, Gradient<Vec2>)>,
{
    type Output = InsertT<C, (Distance<f32>, Gradient<Vec2>)>;

    fn call(self, (ca, cb, t): (C, C, f32)) -> Self::Output {
        let da = ca.clone().get();
        let db = cb.clone().get();

        let mut d = db.lerp(da, t);
        d.0 .0 = d.0 .0 - self.0 * t * (1.0 - t);
        d.1 .0 = d.1 .0.normalize();

        if da.0 < db.0 { ca } else { cb }.insert(d)
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
