//! Combine two shapes using a boolean operation
mod intersection;
mod subtraction;
mod union;

pub use intersection::*;
pub use subtraction::*;
pub use union::*;

use std::{fmt::Debug, marker::PhantomData};

use crate::{Distance, DistanceF32, Domain, DomainT, Identity, Split, SplitT};

use type_fields::t_funk::{
    arrow::{First, Second},
    closure::Compose,
    function::Const,
    function::Id,
    CallF, Closure, Composed, Curry2, Curry2A, EitherUnwrap, Fanout, FanoutT, Fanouted, Firsted,
    Fst, If, LShiftTuple, MakeIf, Seconded, Snd, Split as SplitA, Splitted as SplittedA,
};

/// Combine shapes T, U using the boolean function O
pub struct Boolean<T, U, O>(pub T, pub U, pub PhantomData<O>);

impl<T, U, O> Debug for Boolean<T, U, O>
where
    T: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Boolean")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<T, U, O> Default for Boolean<T, U, O>
where
    T: Default,
    U: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), PhantomData)
    }
}

impl<T, U, O> Clone for Boolean<T, U, O>
where
    T: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<T, U, O> Copy for Boolean<T, U, O>
where
    T: Copy,
    U: Copy,
{
}

impl<T, U, O> Domain<DistanceF32> for Boolean<T, U, O>
where
    T: Domain<DistanceF32>,
    U: Domain<DistanceF32>,
    O: Default,
    DomainT<T, DistanceF32>: Fanout<DomainT<U, DistanceF32>>,
{
    type Domain = Composed<
        Composed<EitherUnwrap, CallF>,
        Composed<Fanouted<MakeIf, O>, FanoutT<DomainT<T, DistanceF32>, DomainT<U, DistanceF32>>>,
    >;

    fn domain(self) -> Self::Domain {
        let d0 = Domain::<DistanceF32>::domain(self.0);
        let d1 = Domain::<DistanceF32>::domain(self.1);

        d0.fanout(d1)
            .compose_l(MakeIf.fanout(O::default()))
            .compose_l(CallF.compose_l(EitherUnwrap))
    }
}

// For distance composite domains, evaluate distance once on its own to determine closest,
// then evaluate the closer domain in full
impl<T, U, O, B> Domain<Split<DistanceF32, B>> for Boolean<T, U, O>
where
    T: Clone + Domain<DistanceF32> + Domain<Split<DistanceF32, B>>,
    U: Clone + Domain<DistanceF32> + Domain<Split<DistanceF32, B>>,
    O: Default,
    DomainT<T, DistanceF32>: Fanout<DomainT<U, DistanceF32>>,
    SplitT<T, DistanceF32, B>: Compose<Fanouted<Id, Id>>,
    SplitT<U, DistanceF32, B>: Compose<Fanouted<Id, Id>>,
{
    type Domain = Composed<
        Snd,
        Composed<
            Seconded<CallF>,
            Composed<
                LShiftTuple,
                Fanouted<
                    Composed<
                        Composed<
                            SplittedA<Composed<EitherUnwrap, CallF>, CallF>,
                            Composed<
                                Fanouted<
                                    Id,
                                    Firsted<
                                        Curry2A<
                                            Const,
                                            If<
                                                DomainT<T, Split<Distance<f32>, B>>,
                                                DomainT<U, Split<Distance<f32>, B>>,
                                            >,
                                        >,
                                    >,
                                >,
                                Composed<
                                    Fanouted<MakeIf, O>,
                                    FanoutT<DomainT<T, Distance<f32>>, DomainT<U, Distance<f32>>>,
                                >,
                            >,
                        >,
                        Fst,
                    >,
                    Id,
                >,
            >,
        >,
    >;

    fn domain(self) -> Self::Domain {
        let d0 = Domain::<DistanceF32>::domain(self.0.clone());
        let d1 = Domain::<DistanceF32>::domain(self.1.clone());

        let p0 = Domain::<Split<DistanceF32, B>>::domain(self.0);
        let p1 = Domain::<Split<DistanceF32, B>>::domain(self.1);

        d0.fanout(d1)
            .compose_l(MakeIf.fanout(O::default()))
            .compose_l(Id.fanout(Const.prefix2(MakeIf.call((p0, p1))).first()))
            .compose_l(CallF.compose_l(EitherUnwrap).split(CallF))
            .compose(Fst)
            .fanout(Id)
            .compose_l(LShiftTuple)
            .compose_l(CallF.second())
            .compose_l(Snd)
    }
}

impl<T, U, O> Domain<Identity> for Boolean<T, U, O> {
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

#[cfg(test)]
mod test {
    use type_fields::t_funk::{
        arrow::First, arrow::Second, closure::Compose, function::Const, CallF, Closure, Curry2,
        EitherUnwrap, Fanout, Fst, Id, LShiftTuple, Lt, MakeIf, Snd, Split as SplitA,
    };

    use crate::{
        shape, DistanceF32, Domain, GradientF32, Isosurface, Point, Position,
        Split, Translate,
    };

    /*
    #[test]
    fn test_boolean() {
        let shape_a = shape() << Point << Isosurface(0.2);
        let shape_b = shape() << Translate(0.1, 0.1) << Point << Isosurface(0.4);

        let d0 = Domain::<DistanceF32>::domain(shape_a);
        let d1 = Domain::<DistanceF32>::domain(shape_b);

        let p0 = Domain::<Split<DistanceF32, GradientF32>>::domain(shape_a);
        let p1 = Domain::<Split<DistanceF32, GradientF32>>::domain(shape_b);

        let f = d0
            .fanout(d1)
            .compose_l(MakeIf.fanout(Lt::default()))
            .compose_l(Id.fanout(Const.prefix2(MakeIf.call((p0, p1))).first()))
            .compose_l(CallF.compose_l(EitherUnwrap).split(CallF))
            .compose(Fst)
            .fanout(Id)
            .compose_l(LShiftTuple)
            .compose_l(CallF.second())
            .compose_l(Snd);

        let res = f.call((Position(0.0, 0.0), Position(0.0, 0.0)));
        panic!("{res:#?}")
    }
    */
}
