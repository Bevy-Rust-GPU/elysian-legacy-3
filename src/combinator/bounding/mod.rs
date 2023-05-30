//! Use one shape as the bounding volume for another

mod inner_bound;
mod outer_bound;

pub use inner_bound::*;
pub use outer_bound::*;

use std::{fmt::Debug, marker::PhantomData};

use crate::{Distance, DistanceF32, Domain, DomainT, Identity};

use type_fields::t_funk::{
    closure::Compose, function::Id, CallF, Composed, Curry2, Curry2B, EitherUnwrap, Fanout,
    Fanouted, MakeIf,
};

/// Combine shapes T, U using the boolean function O
pub struct Bounding<T, U, O>(pub T, pub U, pub PhantomData<O>);

impl<T, U, O> Debug for Bounding<T, U, O>
where
    T: Debug,
    U: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Bounding")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl<T, U, O> Default for Bounding<T, U, O>
where
    T: Default,
    U: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default(), PhantomData)
    }
}

impl<T, U, O> Clone for Bounding<T, U, O>
where
    T: Clone,
    U: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<T, U, O> Copy for Bounding<T, U, O>
where
    T: Copy,
    U: Copy,
{
}

// TODO: Create Split<Distance, D> impl for accelerating domain lookups
//       Will need to use Default to populate D output in fail cases
//
// TODO: Consider generalization to 3D
//       Bounding volume would need to be projected to prevent raymarch convergence on near face
//       i.e. Infinite cone oriented along the eye -> shape vector for a bounding sphere
//
impl<T, U, O> Domain<DistanceF32> for Bounding<T, U, O>
where
    T: Domain<DistanceF32>,
    U: Domain<DistanceF32, Input = T::Input>,
    O: Default,
    DomainT<T, DistanceF32>: Fanout<DomainT<U, DistanceF32>>,
{
    type Input = T::Input;
    type Domain = Composed<
        Composed<EitherUnwrap, CallF>,
        Fanouted<
            Composed<Curry2B<MakeIf, Distance<f32>>, <U as Domain<Distance<f32>>>::Domain>,
            Composed<Curry2B<O, Distance<f32>>, <T as Domain<Distance<f32>>>::Domain>,
        >,
    >;

    fn domain(self) -> Self::Domain {
        let d0 = Domain::<DistanceF32>::domain(self.0);
        let d1 = Domain::<DistanceF32>::domain(self.1);

        (d1.compose_l(MakeIf.suffix2(Distance(f32::INFINITY))))
            .fanout(d0.compose_l(O::default().suffix2(Distance(0.0))))
            .compose_l(CallF.compose_l(EitherUnwrap))
    }
}

impl<T, U, O> Domain<Identity> for Bounding<T, U, O> {
    type Input = ();
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

#[cfg(test)]
mod test {
    use type_fields::t_funk::{
        closure::Compose, CallF, Closure, Curry2, EitherUnwrap, Fanout, Lt, MakeIf,
    };

    use crate::{shape, Circle, Distance, DistanceF32, Domain, Position, Translate};

    #[test]
    fn test_bounding() {
        let shape_a = shape() << Circle(0.5);
        let shape_b = shape() << Translate(-0.25, 0.0) << Circle(0.5);

        let d0 = Domain::<DistanceF32>::domain(shape_b);
        let d1 = Domain::<DistanceF32>::domain(shape_a);

        let f = (d1.compose_l(MakeIf.prefix2(Distance(f32::INFINITY))))
            .fanout(d0.compose_l(Lt::default().suffix2(Distance(0.0))))
            .compose_l(CallF.compose_l(EitherUnwrap));

        let _res = f.call(Position(0.0, 0.0));
    }
}
