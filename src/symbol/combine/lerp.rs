use crate::glam::{Vec2, Vec3, Vec4};
use t_funk::macros::types;

use crate::{Color, Distance, Gradient};

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

impl Lerp<Vec2, f32> for Vec2 {
    type Lerp = Self;

    fn lerp(self, b: Vec2, t: f32) -> Self::Lerp {
        self.lerp(b, t)
    }
}

impl Lerp<Vec3, f32> for Vec3 {
    type Lerp = Self;

    fn lerp(self, b: Vec3, t: f32) -> Self::Lerp {
        self.lerp(b, t)
    }
}

impl Lerp<Vec4, f32> for Vec4 {
    type Lerp = Self;

    fn lerp(self, b: Vec4, t: f32) -> Self::Lerp {
        self.lerp(b, t)
    }
}

impl<T, U> Lerp<Distance<T>, U> for Distance<T>
where
    T: Lerp<T, U, Lerp = T>,
{
    type Lerp = Self;

    fn lerp(self, b: Distance<T>, t: U) -> Self::Lerp {
        Distance(self.0.lerp(b.0, t))
    }
}

impl<T, U> Lerp<Gradient<T>, U> for Gradient<T>
where
    T: Lerp<T, U, Lerp = T>,
{
    type Lerp = Self;

    fn lerp(self, b: Gradient<T>, t: U) -> Self::Lerp {
        Gradient(self.0.lerp(b.0, t))
    }
}

impl<T, U> Lerp<Color<T>, U> for Color<T>
where
    T: Lerp<T, U, Lerp = T>,
{
    type Lerp = Self;

    fn lerp(self, b: Color<T>, t: U) -> Self::Lerp {
        Color(self.0.lerp(b.0, t))
    }
}
