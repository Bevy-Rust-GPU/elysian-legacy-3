//! Evaluation domain
//!
//! Subdomain of distance, composes with Subtree to create an evaluation context

use crate::{DistanceF, DistanceF32, DistanceT, Domain, DomainF, Subtree, SubtreeF, SubtreeT};

use type_fields::t_funk::{Closure, Split, SplitT};

pub enum Evaluate {}

pub type EvaluateT<T> = <T as Domain<Evaluate>>::Domain;
pub type EvaluateF = DomainF<Evaluate>;

impl<T> Domain<Evaluate> for T
where
    T: Clone + Domain<DistanceF32> + Domain<Subtree>,
    DistanceT<T>: Split<SubtreeT<T>>,
{
    type Domain = SplitT<DistanceT<T>, SubtreeT<T>>;

    fn domain(self) -> Self::Domain {
        DistanceF::default()
            .call(self.clone())
            .split(SubtreeF::default().call(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{shape, Isosurface, Point, Translate, Domain, Evaluate};

    #[test]
    fn test_evaluate() {
        let shape_a = shape() << Translate(-0.8, -0.8) << Point << Isosurface(0.4);
        let shape_b = shape() << Translate(0.8, 0.8) << Point << Isosurface(0.2);
        let shape_c = shape() << Translate(0.0, 0.8) << Point << Isosurface(0.5);
        let shape_d = shape() << Translate(0.0, -0.8) << Point << Isosurface(0.3);

        let shape = shape_a + shape_b + shape_c * shape_d;

        let res = Domain::<Evaluate>::domain(shape);
        panic!("{res:#?}");
    }
}
