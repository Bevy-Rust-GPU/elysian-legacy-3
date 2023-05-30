use crate::{
    Distance, DistanceF32, Domain, DomainT, Gradient, GradientF32, Identity, PositionF32, Split, impl_domains,
};
use type_fields::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::{
        closure::{Closure, Compose},
        function::Id,
        Abs, Composed, Curry2, Fanout, Fanouted, FmapF, Fst, Function, Snd,
    },
};

// Isosurface output modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Manifold;

impl Domain<DistanceF32> for Manifold {
    type Input = DistanceF32;
    type Domain = ManifoldDistance;

    fn domain(self) -> Self::Domain {
        ManifoldDistance
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldDistance;

impl Function<DistanceF32> for ManifoldDistance {
    type Output = DistanceF32;

    fn call(input: DistanceF32) -> Self::Output {
        FmapF.suffix2(Abs).call(input)
    }
}

/*
impl<T> Domain<Split<DistanceF32, T>> for Manifold
where
    Manifold: Domain<T>,
{
    type Domain =
        type_fields::t_funk::arrow::SplitT<DomainT<Manifold, DistanceF32>, DomainT<Manifold, T>>;

    fn domain(self) -> Self::Domain {
        Domain::<DistanceF32>::domain(self).split(Domain::<T>::domain(self))
    }
}
*/

impl Domain<GradientF32> for Manifold {
    type Input = (DistanceF32, GradientF32);
    type Domain = ManifoldGradient;

    fn domain(self) -> Self::Domain {
        ManifoldGradient
    }
}

impl Domain<Split<DistanceF32, GradientF32>> for Manifold {
    type Input = ();
    type Domain = Fanouted<Composed<ManifoldDistance, Fst>, ManifoldGradient>;

    fn domain(self) -> Self::Domain {
        Fst.compose_l(ManifoldDistance).fanout(ManifoldGradient)
    }
}

impl<T> Domain<Split<DistanceF32, Split<GradientF32, T>>> for Manifold
where
    Manifold: Domain<T>,
{
    type Input = ();
    type Domain = Fanouted<
        Composed<DomainT<Manifold, DistanceF32>, Fst>,
        Fanouted<
            Composed<ManifoldGradient, Fanouted<Fst, Composed<Fst, Snd>>>,
            Composed<DomainT<Manifold, T>, Composed<Snd, Snd>>,
        >,
    >;

    fn domain(self) -> Self::Domain {
        Fst.compose_l(Domain::<DistanceF32>::domain(self)).fanout(
            Fst.fanout(Snd.compose_l(Fst))
                .compose_l(ManifoldGradient)
                .fanout(Snd.compose_l(Snd).compose_l(Domain::<T>::domain(self))),
        )
    }
}

impl Domain<Identity> for Manifold {
    type Input = ();
    type Domain = Id;

    fn domain(self) -> Self::Domain {
        Id
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldGradient;

impl Function<(DistanceF32, GradientF32)> for ManifoldGradient {
    type Output = GradientF32;

    fn call((Distance(d), Gradient(x, y)): (DistanceF32, GradientF32)) -> Self::Output {
        let s = d.signum();
        Gradient(x * s, y * s)
    }
}

impl_domains!(Manifold);

#[cfg(test)]
mod test {
    use image::{DynamicImage, ImageBuffer, Pixel, Rgb};
    use type_fields::t_funk::Closure;
    use viuer::Config;

    use crate::{
        shape, Distance, DistanceF32, Domain, Gradient, GradientF32, Identity, Isosurface, Lift,
        Manifold, Point, Position, Split as SplitDomain, Translate,
    };

    /*
    #[test]
    fn test_domain_combination() {
        type TestDomain = SplitDomain<DistanceF32, SplitDomain<GradientF32, Identity>>;

        let pure = Domain::<TestDomain>::domain(Lift);
        let input = Domain::<TestDomain>::domain(Translate(0.01, -0.01));
        let field = Domain::<TestDomain>::domain(Point);
        let output1 = Domain::<TestDomain>::domain(Isosurface(0.35));
        let output2 = Domain::<TestDomain>::domain(Manifold);
        let output3 = Domain::<TestDomain>::domain(Isosurface(0.1));

        let shape = shape()
            << Lift
            << Translate(0.01, -0.01)
            << Point
            << Isosurface(0.35)
            << Manifold
            << Isosurface(0.1);
        let f =
            Domain::<SplitDomain<DistanceF32, SplitDomain<GradientF32, Identity>>>::domain(shape);

        /*
        let f = pure
            .compose_l(input)
            .compose_l(field)
            .compose_l(output1)
            .compose_l(output2)
            .compose_l(output3);
        */

        let pure = pure.call(Position(0.0, 0.0));
        let input = input.call(pure);
        let field = field.call(input);
        let output1 = output1.call(field);
        let output2 = output2.call(output1);
        let _output3 = output3.call(output2);

        let mut out = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(32, 32);
        for y in 0..32 {
            for x in 0..32 {
                let nx = x as f32 / 32.0;
                let nx = nx - 0.5;
                let ny = y as f32 / 32.0;
                let ny = ny - 0.5;

                let res = f.call(Position(nx, ny));
                let (Distance(dist), (Gradient(gx, gy), _)) = res;
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
    */
}
