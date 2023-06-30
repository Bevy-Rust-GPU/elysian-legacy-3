use crate::glam::{DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{functions, impl_adt, types},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Alias, Combine, Domains, Modify, Run};

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
    {
        type LiftParam = Self;

        fn lift_param(self, _: C) -> Self::LiftParam {
            self
        }
    }
}

#[cfg(feature = "std")]
impl<C> LiftParam<C> for String {
    type LiftParam = Self;

    fn lift_param(self,_:C) -> Self::LiftParam {
        self
    }
}

impl_adt! {
    impl<A, B> LiftParam<B> for Run<A> | Modify<A> | Domains<A> | Alias<A>
    where
        A: Fmap<Curry2B<LiftParamF, B>>,
    {
        type LiftParam = This<FmapT<A, Curry2B<LiftParamF, B>>>;

        fn lift_param(self, input: B) -> Self::LiftParam {
            This(self.0.fmap(LiftParamF.suffix2(input)))
        }
    }
}

impl<A, B, F, C> LiftParam<C> for Combine<A, B, F>
where
    A: Fmap<Curry2B<LiftParamF, C>>,
    B: Fmap<Curry2B<LiftParamF, C>>,
    F: Fmap<Curry2B<LiftParamF, C>>,
    C: Clone,
{
    type LiftParam = Combine<
        FmapT<A, Curry2B<LiftParamF, C>>,
        FmapT<B, Curry2B<LiftParamF, C>>,
        FmapT<F, Curry2B<LiftParamF, C>>,
    >;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Combine(
            self.0.fmap(LiftParamF.suffix2(input.clone())),
            self.1.fmap(LiftParamF.suffix2(input.clone())),
            self.2.fmap(LiftParamF.suffix2(input)),
        )
    }
}
