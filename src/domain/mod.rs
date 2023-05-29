mod distance;
mod evaluate;
mod gradient;
mod identity;
mod null;
mod position;
mod split;
mod subtree;
mod context {
    #[cfg(test)]
    mod test {
        use std::marker::PhantomData;

        use crate::{
            shape, Distance, DistanceF32, Domain, DomainT, Gradient, GradientF32,
            Isosurface, Manifold, Point, Position, PositionF32, Shape, Translate, TranslateF,
        };
        use image::{DynamicImage, ImageBuffer, Pixel, Rgb};
        use type_fields::{
            macros::{arrow::Arrow, category::Category, functions, Closure},
            t_funk::{
                arrow::{Fanout, Fanouted, First},
                closure::{Closure, Compose, ComposeLF, Composed, Curried2, Curry2, Flip, OutputT},
                collection::set::{Get, GetF, LiftContext, LiftGet, LiftSet, Set, SetF},
                function::{Const, Id},
                hlist::{Chain, Cons, Nil},
                CallF, Curry2A, FanoutF, Flipped, Fmap, FmapT, Function,
            },
        };
        use viuer::Config;

        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        struct Ctx {
            pos: PositionF32,
            dist: DistanceF32,
            grad: GradientF32,
        }

        impl Get<PositionF32> for Ctx {
            fn get(self) -> PositionF32 {
                self.pos
            }
        }

        impl Get<DistanceF32> for Ctx {
            fn get(self) -> DistanceF32 {
                self.dist
            }
        }

        impl Get<GradientF32> for Ctx {
            fn get(self) -> GradientF32 {
                self.grad
            }
        }

        impl Set<PositionF32> for Ctx {
            fn set(self, t: PositionF32) -> Self {
                Self { pos: t, ..self }
            }
        }

        impl Set<DistanceF32> for Ctx {
            fn set(self, t: DistanceF32) -> Self {
                Self { dist: t, ..self }
            }
        }

        impl Set<GradientF32> for Ctx {
            fn set(self, t: GradientF32) -> Self {
                Self { grad: t, ..self }
            }
        }

        pub trait Interpret<A, B> {
            type Interpret;

            fn interpret(self) -> Self::Interpret;
        }

        impl<T, A, B, AI, BI> Interpret<(A, B), (AI, BI)> for T
        where
            T: Clone + Domain<A> + Interpret<B, BI>,
        {
            type Interpret = Composed<
                type_fields::t_funk::closure::ComposeLF,
                Fanouted<
                    Composed<
                        Curried2<Flipped<type_fields::t_funk::SetF>>,
                        Composed<<T as Domain<A>>::Domain, GetF<AI>>,
                    >,
                    <T as Interpret<B, BI>>::Interpret,
                >,
            >;

            fn interpret(self) -> Self::Interpret {
                Domain::<A>::domain(self.clone())
                    .lift_context()
                    .fanout(self.interpret())
                    .compose_l(ComposeLF)
            }
        }

        impl<T> Interpret<(), ()> for T {
            type Interpret = Composed<
                Curried2<Flipped<type_fields::t_funk::SetF>>,
                Composed<(), type_fields::t_funk::GetF<()>>,
            >;

            fn interpret(self) -> Self::Interpret {
                ().lift_context()
            }
        }

        pub type InterpretT<T, D, P> = <T as Interpret<D, P>>::Interpret;

        #[derive(Closure, Category, Arrow)]
        pub struct InterpretF<D, P>(PhantomData<(D, P)>);

        impl<D, P> Default for InterpretF<D, P> {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl<D, P> Clone for InterpretF<D, P> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<D, P, T> Function<T> for InterpretF<D, P>
        where
            T: Interpret<D, P>,
        {
            type Output = InterpretT<T, D, P>;

            fn call(input: T) -> Self::Output {
                input.interpret()
            }
        }

        pub trait InterpretShape<D, P> {
            type InterpretShape;

            fn interpret_shape(self) -> Self::InterpretShape;
        }

        impl<T, D, P> InterpretShape<D, P> for Shape<T>
        where
            T: Fmap<Composed<CallF, Fanouted<InterpretF<D, P>, Id>>>,
            FmapT<T, Composed<CallF, Fanouted<InterpretF<D, P>, Id>>>: Chain,
        {
            type InterpretShape = <<T as Fmap<
                Composed<CallF, Fanouted<InterpretF<D, P>, Id>>,
            >>::Fmap as Chain>::Chain;

            fn interpret_shape(self) -> Self::InterpretShape {
                self.0
                    .fmap(InterpretF::<D, P>::default().fanout(Id).compose_l(CallF))
                    .chain()
            }
        }

        #[test]
        fn test_context() {
            let context = Ctx::default();
            let context = Set::<PositionF32>::set(context, Position(0.5, 0.5));

            let translate = Translate(0.02, 0.04);
            let point = Point;
            let isosurface = Isosurface(0.2);
            let manifold = Manifold;
            let shape = shape() << translate << point << isosurface << manifold;

            let foo = shape.0.fmap(InterpretF::<
                (DistanceF32, (GradientF32, ())),
                (Ctx, (Ctx, ())),
            >::default());
            //.fmap(FanoutF.suffix2(Id))
            //.fmap(ComposeLF.suffix2(CallF))
            //.chain();

            //let res = foo.call(context);

            let combo_a = Interpret::<
                (DistanceF32, (GradientF32, ())),
                (PositionF32, (PositionF32, ())),
            >::interpret(shape.0 .0);

            let res = combo_a.call(context);
            //panic!("{res:#?}");

            let combo_b =
                Interpret::<(DistanceF32, (GradientF32, ())), _>::interpret(shape.0 .1 .0);

            let res = combo_b.call(context);
            //panic!("{res:#?}");

            let combo_c =
                Interpret::<(DistanceF32, (GradientF32, ())), _>::interpret(shape.0 .1 .1 .0);

            let res = combo_c.call(context);
            //panic!("{res:#?}");

            let combo_d =
                Interpret::<(DistanceF32, (GradientF32, ())), _>::interpret(shape.0 .1 .1 .1 .0);

            let res = combo_d.call(context);
            //panic!("{res:#?}");

            let combo_domain = (combo_a.fanout(Id).compose_l(CallF))
                .compose_l(combo_b.fanout(Id).compose_l(CallF))
                .compose_l(combo_c.fanout(Id).compose_l(CallF))
                .compose_l(combo_d.fanout(Id).compose_l(CallF));

            let mut out = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(32, 32);
            for y in 0..32 {
                for x in 0..32 {
                    let nx = x as f32 / 32.0;
                    let nx = nx - 0.5;
                    let ny = y as f32 / 32.0;
                    let ny = ny - 0.5;

                    let res = combo_domain.call(Ctx {
                        pos: Position(nx, ny),
                        ..Default::default()
                    });
                    let Ctx {
                        dist: Distance(dist),
                        grad: Gradient(gx, gy),
                        ..
                    } = res;
                    let dist = 1.0 - (dist * 40.0).max(0.0).min(1.0);

                    out.put_pixel(x, y, *Pixel::from_slice(&[gx, gy, dist]));
                }
            }

            viuer::print(
                &DynamicImage::from(DynamicImage::from(out).to_rgb8()),
                &Config {
                    absolute_offset: false,
                    width: Some(32),
                    use_iterm: false,
                    ..Default::default()
                },
            )
            .unwrap();
        }
    }
}

pub use context::*;
pub use distance::*;
pub use evaluate::*;
pub use gradient::*;
pub use identity::*;
pub use position::*;
pub use split::*;
pub use subtree::*;

use type_fields::macros::functions;

#[functions]
pub trait Domain<T>
{
    type Domain;

    fn domain(self) -> Self::Domain;
}

pub type DomainT<T, D> = <T as Domain<D>>::Domain;
