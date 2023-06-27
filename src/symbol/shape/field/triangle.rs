use glam::Vec2;
use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

use crate::{
    Alias, Demanifold, ExpandAlias, ExpandAliasT, IntoMonad, IntoMonadT, LiftAdt, Line, Reflect,
    Translate,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Triangle<T>(pub T);

impl<T, F> Fmap<F> for Triangle<T>
where
    F: Closure<T>,
{
    type Fmap = Triangle<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Triangle(f.call(self.0))
    }
}

impl<T> IntoMonad for Triangle<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Triangle<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

const FRAC_TAU_3: f32 = core::f32::consts::TAU / 3.0;

impl<D> ExpandAlias<D> for Triangle<f32> {
    type ExpandAlias = (
        Reflect<
            IntoMonadT<
                Reflect<MappendT<(Translate<Vec2>,), (Demanifold<ExpandAliasT<Line<Vec2>, D>>,)>>,
            >,
        >,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        let angle = FRAC_TAU_3;
        let radius = self.0;
        let width = radius * core::f32::consts::FRAC_PI_3.sin();
        let line = ExpandAlias::<D>::expand_alias(Line(Vec2::new(width, 0.0)));

        (Reflect(
            Vec2::X,
            Reflect(
                Vec2::new(angle.cos(), angle.sin()),
                (Translate(Vec2::Y * width / 3.0_f32.sqrt()),)
                    .mappend((Demanifold(Vec2::Y, line),)),
            )
            .into_monad(),
        ),)
    }
}
