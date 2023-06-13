use glam::{DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{functions, impl_adt, types},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{AdtEnd, Combine, Run, Then};

#[functions]
#[types]
pub trait LiftParam<C> {
    type LiftParam;

    fn lift_param(self, input: C) -> Self::LiftParam;
}

impl_adt! {
    impl<C> LiftParam<C> for
        bool
            | u8
            | u16
            | u32
            | u64
            | i8
            | i16
            | i32
            | i64
            | f32
            | f64
            | String
            | Vec2
            | Vec3
            | Vec4
            | DVec2
            | DVec3
            | DVec4
            | UVec2
            | UVec3
            | UVec4
            | IVec2
            | IVec3
            | IVec4
            | AdtEnd
    {
        type LiftParam = Self;

        fn lift_param(self, _: C) -> Self::LiftParam {
            self
        }
    }
}

impl<A, B> LiftParam<B> for Run<A>
where
    A: Fmap<Curry2B<LiftParamF, B>>,
{
    type LiftParam = Run<FmapT<A, Curry2B<LiftParamF, B>>>;

    fn lift_param(self, input: B) -> Self::LiftParam {
        Run(self.0.fmap(LiftParamF.suffix2(input)))
    }
}

impl<A, B, C> LiftParam<C> for Then<A, B>
where
    A: LiftParam<C>,
    B: LiftParam<C>,
    C: Clone,
{
    type LiftParam = Then<LiftParamT<A, C>, LiftParamT<B, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Then(self.0.lift_param(input.clone()), self.1.lift_param(input))
    }
}

impl<A, B, F, C> LiftParam<C> for Combine<A, B, F>
where
    A: LiftParam<C>,
    B: LiftParam<C>,
    C: Clone,
{
    type LiftParam = Combine<LiftParamT<A, C>, LiftParamT<B, C>, F>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Combine(
            self.0.lift_param(input.clone()),
            self.1.lift_param(input),
            self.2,
        )
    }
}
