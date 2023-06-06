use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{functions, impl_adt, types},
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Combine, Field, Input, Modify, Nil, Output, Sequence, Shape};

#[functions]
#[types]
pub trait LiftParam<C> {
    type LiftParam;

    fn lift_param(self, input: C) -> Self::LiftParam;
}

impl_adt! {
    impl<A, B, C> LiftParam<C> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        A: Fmap<Curry2B<LiftParamF, C>>,
        B: LiftParam<C>,
        C: Clone,
    {
        type LiftParam = This<FmapT<A, Curry2B<LiftParamF, C>>, LiftParamT<B, C>>;

        fn lift_param(self, input: C) -> Self::LiftParam {
            This(
                self.0.fmap(LiftParamF.suffix2(input.clone())),
                self.1.lift_param(input),
            )
        }
    }
}

impl<C> LiftParam<C> for Nil {
    type LiftParam = Self;

    fn lift_param(self, _: C) -> Self::LiftParam {
        self
    }
}

impl<T, C> LiftParam<C> for Shape<T>
where
    T: LiftParam<C>,
{
    type LiftParam = Shape<LiftParamT<T, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Shape(self.0.lift_param(input))
    }
}

impl<T, C> LiftParam<C> for Modify<T>
where
    T: Fmap<Curry2B<LiftParamF, C>>,
{
    type LiftParam = Modify<FmapT<T, Curry2B<LiftParamF, C>>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Modify(self.0.fmap(LiftParamF.suffix2(input)))
    }
}

impl<A, B, C> LiftParam<C> for Sequence<A, B>
where
    A: LiftParam<C>,
    B: LiftParam<C>,
    C: Clone,
{
    type LiftParam = Sequence<LiftParamT<A, C>, LiftParamT<B, C>>;

    fn lift_param(self, input: C) -> Self::LiftParam {
        Sequence(self.0.lift_param(input.clone()), self.1.lift_param(input))
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
