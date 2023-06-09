//! Elysian ADT
//! Elysian = Input a
//! | Field b
//! | Output c
//! | Modify d
//! | Sequence [In|Field|Out|Modify]
//! | Combine Field|Shape|Combine Field|Shape|Combine f
//! where
//!   a: InputModifer
//!   b: FieldMorphism
//!   c: OutputModifier
//!   f: CombineFunction
//!
//! Example:
//!
//! Shape [
//!   In Translate -0.1 -0.3,
//!   Combine (
//!     Shape [
//!       In Translate 0.2 0.2,
//!       Field Point,
//!       Out Isosurface 0.3,
//!     ],
//!     Shape [
//!       In Translate -0.2 -0.2,
//!       Field Point,
//!       Out Isosurface 0.5,
//!       Out Manifold,
//!     ],
//!     Boolean(Lt),
//!   ),
//!   Out Isosurface 0.2,
//! ]
//!

mod algebra;
mod impls;
mod shape;

pub use algebra::*;
pub use impls::*;
pub use shape::*;

mod bounds;
pub(crate) use bounds::*;

use t_funk::macros::{define_adt, Copointed, Pointed};

define_adt!(
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Pointed, Copointed)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ADT
      // Run a computation
      = Run<A>(pub A)
      // Sequence two computations
      | Then<A, B>(pub A, pub B)
      // Combine two computations
      | Combine<A, B, F>(pub A, pub B, pub F)
      // Terminating type
      | AdtEnd;
);

pub use t_funk::op_chain::Done;

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::{Closure, Compose, ComposeLF, Curry2},
        collection::{
            hlist::{Cons, Nil},
            set::LiftContextF,
        },
        function::{Id, PrintLn},
        macros::lift,
        typeclass::{
            applicative::Apply,
            arrow::{Fanout, FanoutF},
            copointed::CopointF,
            functor::{Fmap, FmapF},
            monad::Chain,
            monoid::{Endo, MconcatF},
        },
    };

    use crate::{
        adt, intersection, union, AdtEnd, Ascii, Circle, Combine, ContextGet, Dist, Distance, Done,
        Evaluate, Gradient, LiftCombine, LiftDomainFunctionF, /*LiftDomainFunctions,*/ LiftDomainsF,
        LiftEvaluate, LiftParam, PosDist, PosDistGrad, Rasterize, Run, ShapeEnd, Then, Translate,
        ASCII_RAMP,
    };

    #[test]
    fn test_adt() {
        let shape_a = adt() << Translate(Vec2::new(-0.8, -0.8)) << Circle(0.2_f32) >> Done;
        let shape_b = adt() << Translate(Vec2::new(0.8, 0.8)) << Circle(0.1_f32) >> Done;
        let shape_c = adt() << Translate(Vec2::new(0.0, 0.8)) << Circle(0.3_f32) >> Done;
        let shape_d = adt() << Translate(Vec2::new(0.0, -0.8)) << Circle(0.15_f32) >> Done;

        let combined =
            union() << shape_a << shape_b << shape_c >> intersection() << shape_d >> Done;

        /*
        let foo = adt() << combined << ContextGet::<Distance<f32>>::default() >> Done;

        let _foo = Evaluate::<Dist<f32>, PosDist<Vec2, f32>>::evaluate(
            foo,
            PosDist::<Vec2, f32>::default(),
        );
        */

        #[lift]
        fn make_cons<A>(a: A) -> Cons<A, Nil> {
            Cons(a, Nil)
        }

        #[lift]
        fn make_cons_pair<A, B>(a: A, b: B) -> Cons<A, Cons<B, Nil>> {
            Cons(a, Cons(b, Nil))
        }

        /*
        // Lift parameters so functions will be valid on creation
        let _foo = combined.lift_param(PosDist::<Vec2, f32>::default());

        // Lift symbols into sets of functions
        let _foo = _foo.fmap(FmapF.suffix2(LiftDomainFunctions::<
            Distance<f32>,
            LiftDomainFunctions<Gradient<Vec2>, ()>,
        >::default()));

        // Lift functions to context I/O
        let _foo =
            _foo.fmap(FmapF.suffix2(FmapF.suffix2(
                LiftContextF::<PosDistGrad<Vec2, f32, Vec2>>::default(),
            )));

        // Fold sets of functions into a fan structure that takes a single context

        // Apply context to fan structure to create setters

        // Compose setters
        
        // Apply setters to create new context
        */

        /*
        #[lift]
        fn make_endo<A>(a: A) -> Endo<A> {
            Endo(a)
        }

        let _foo = _foo.lift_combine();
        */

        //panic!("{_foo:#?}");

        /*
        let _foo = LiftEvaluate::<(Distance<f32>, ())>::lift_evaluate(
            combined
                .lift_param(PosDist::<Vec2, f32>::default())
                .lift_combine(),
        )
        .call(PosDist::<Vec2, f32>::default());

        let _foo = Rasterize::<(Distance<f32>, ())>::default().call((PosDist::default(), combined));

        Rasterize::<(Distance<f32>, ())> {
            width: 32,
            height: 32,
            ..Default::default()
        }
        .compose_l(Ascii.prefix2(ASCII_RAMP))
        .compose_l(PrintLn)
        .call((PosDist::default(), combined));
        */
    }
}
