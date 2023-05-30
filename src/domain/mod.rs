mod distance;
mod evaluate;
mod gradient;
mod identity;
mod null;
mod position;
mod split;
mod subtree;
mod context {
    use crate::Shape;

    use type_fields::{
        macros::functions,
        t_funk::{
            closure::ComposeLT,
            closure::{Compose, ComposeLF, Composed, Curry2, Curry2B},
            foldable::{Foldl, FoldlT, Foldr, FoldrT},
            function::{Const, Id},
            hlist::{Chain, ChainT, Cons, Nil},
            CallF, Curry2A, Fanout, FanoutF, FanoutT, Fmap, FmapT, Function,
        },
    };

    #[functions]
    pub trait Domains<A> {
        type Domains;

        fn domains(self) -> Self::Domains;
    }

    #[macro_export]
    macro_rules! impl_domains {
            ($ident:ident $(<$gen:ident>)?) => {
                impl<A, B, $($gen)?> elysian::Domains<(A, B)> for $ident $(<$gen>)?
                where
                    $ident $(<$gen>)?: Clone + elysian::Domain<A> + elysian::Domains<B>,
                {
                    type Domains = type_fields::t_funk::closure::Composed<
                        type_fields::t_funk::closure::ComposeLF,
                        type_fields::t_funk::arrow::Fanouted<
                            type_fields::t_funk::closure::Composed<type_fields::t_funk::closure::Curried2<type_fields::t_funk::closure::Flipped<type_fields::t_funk::set::SetF>>, type_fields::t_funk::closure::Composed<elysian::DomainT<$ident $(<$gen>)?, A>, type_fields::t_funk::set::GetF<elysian::InputT<$ident $(<$gen>)?, A>>>>,
                            elysian::DomainsT<$ident $(<$gen>)?, B>,
                        >,
                    >;

                    fn domains(self) -> Self::Domains {
                        type_fields::t_funk::closure::Compose::compose_l(
                            type_fields::t_funk::arrow::Fanout::fanout(
                                type_fields::t_funk::set::LiftContext::<_>::lift_context(
                                    Domain::<A>::domain(self.clone())
                                ),
                                self.domains()
                            ),
                            type_fields::t_funk::closure::ComposeLF
                        )
                    }
                }

                impl<A, $($gen)?> elysian::Domains<(A, ())> for $ident $(<$gen>)?
                where
                    $ident $(<$gen>)?: Clone + elysian::Domain<A>,
                {
                    type Domains = type_fields::t_funk::closure::Composed<type_fields::t_funk::closure::Curried2<type_fields::t_funk::closure::Flipped<type_fields::t_funk::set::SetF>>, type_fields::t_funk::closure::Composed<elysian::DomainT<$ident $(<$gen>)?, A>, type_fields::t_funk::set::GetF<elysian::InputT<$ident $(<$gen>)?, A>>>>;

                    fn domains(self) -> Self::Domains {
                        type_fields::t_funk::set::LiftContext::<_>::lift_context(Domain::<A>::domain(self.clone()))
                    }
                }
            };
        }

    pub type DomainsT<T, D> = <T as Domains<D>>::Domains;

    impl<T, D> Domains<D> for Shape<T>
    where
        T: Fmap<DomainsF<D>>,
        FmapT<T, DomainsF<D>>: Chain,
    {
        type Domains = ChainT<FmapT<T, DomainsF<D>>>;

        fn domains(self) -> Self::Domains {
            self.0.fmap(DomainsF::<D>::default()).chain()
        }
    }

    #[functions]
    pub trait Fan {
        type Fan;

        fn fan(self) -> Self::Fan;
    }

    type FanT<T> = <T as Fan>::Fan;

    impl<A, B> Fan for Cons<A, B>
    where
        A: Fanout<FanT<B>>,
        B: Fan,
    {
        type Fan = Composed<ComposeLF, FanoutT<A, FanT<B>>>;

        fn fan(self) -> Self::Fan {
            self.0.fanout(self.1.fan()).compose_l(ComposeLF)
        }
    }

    impl<A> Fan for Cons<A, Nil> {
        type Fan = A;

        fn fan(self) -> Self::Fan {
            self.0
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        use elysian::Point;

        use crate::{
            shape, DistanceF32, DomainsF, GradientF32, Isosurface, PosDistGrad, Translate, Rasterize, Ascii, ASCII_RAMP,
        };

        use type_fields::t_funk::{
            arrow::FanoutF,
            closure::{Closure, Compose, ComposeF, ComposeLF, Curry2},
            function::{Const, Id},
            Curry2A, Fanout, Flip, Flipped, Foldr, Split, PrintLn,
        };

        #[test]
        fn test_ascii() {
            let shape_a = shape() << Translate(0.2, 0.2) << Point;
            let shape_b = shape() << Point << Isosurface(0.2);
            let union = shape_a + shape_b;
            let shape = shape() << Translate(0.2, 0.4) << union << Isosurface(0.2);

            let domains = Domains::<(DistanceF32, (GradientF32, ()))>::domains(shape_a);
            panic!("{domains:#?}");

            /*
            let res = domains
                .call(PosDistGrad::default());
            panic!("{res:#?}");

            let rast = Rasterize::<(DistanceF32, ()), PosDistGrad>::default()
                .compose_l(Ascii.prefix2(ASCII_RAMP))
                .compose_l(PrintLn)
                .call(shape_b);

            panic!("{rast:#?}");
            */
        }
    }

    #[cfg(test)]
    mod test_ {
        use image::{DynamicImage, ImageBuffer, Pixel, Rgb};
        use type_fields::t_funk::closure::Closure;
        use viuer::Config;

        use crate::{
            shape, Distance, DistanceF32, Domains, Gradient, GradientF32, Isosurface, Manifold,
            Point, PosDistGrad, Position, Translate,
        };

        #[test]
        fn test_context() {
            /*
            let shape_a = shape() << Translate(0.02, 0.04) << Point << Isosurface(0.2) << Manifold;
            let shape_b = shape() << Translate(0.02, 0.04) << Point << Isosurface(0.1);
            let composite = shape_a + shape_b;
            let f = Domains::<(DistanceF32, (GradientF32, ()))>::domains(composite);

            let mut out = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(32, 32);
            for y in 0..32 {
                for x in 0..32 {
                    let nx = x as f32 / 32.0;
                    let nx = nx - 0.5;
                    let ny = y as f32 / 32.0;
                    let ny = ny - 0.5;

                    let res = f.call(PosDistGrad {
                        pos: Position(nx, ny),
                        ..Default::default()
                    });

                    let PosDistGrad {
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
            */
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
pub trait Domain<T> {
    type Input;
    type Domain;

    fn domain(self) -> Self::Domain;
}

pub type DomainT<T, D> = <T as Domain<D>>::Domain;
pub type InputT<T, D> = <T as Domain<D>>::Input;
