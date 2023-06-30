use std::marker::PhantomData;

use image::DynamicImage;
use t_funk::{
    closure::Closure,
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};
use viuer::Config;

use crate::{EvaluateFunction, EvaluateInputs, IntoMonad, IntoTuple, IntoTupleT, LiftAdt, Modify};

pub trait Viuer: Sized + IntoTuple {
    type Viuer<U>
    where
        ViuerPrinter<U>: IntoTuple,
        IntoTupleT<Self>: Mappend<IntoTupleT<ViuerPrinter<U>>>;

    fn viuer<U>(self) -> Self::Viuer<U>
    where
        ViuerPrinter<U>: IntoTuple,
        IntoTupleT<Self>: Mappend<IntoTupleT<ViuerPrinter<U>>>;
}

impl<T> Viuer for T
where
    T: IntoTuple,
{
    type Viuer<U> = MappendT<IntoTupleT<T>, IntoTupleT<ViuerPrinter<U>>>
        where
            ViuerPrinter<U>: IntoTuple,
            IntoTupleT<T>: Mappend<IntoTupleT<ViuerPrinter<U>>>;

    fn viuer<U>(self) -> Self::Viuer<U>
    where
        ViuerPrinter<U>: IntoTuple,
        IntoTupleT<T>: Mappend<IntoTupleT<ViuerPrinter<U>>>,
    {
        self.into_tuple()
            .mappend(ViuerPrinter::<U>::default().into_tuple())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViuerPrinter<T> {
    pub transparent: bool,
    pub absolute_offset: bool,
    pub x: u16,
    pub y: i16,
    pub restore_cursor: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub truecolor: bool,
    pub use_kitty: bool,
    pub use_iterm: bool,
    pub image_buffer: PhantomData<T>,
}

impl<T> Default for ViuerPrinter<T> {
    fn default() -> Self {
        Self {
            transparent: true,
            absolute_offset: false,
            x: 0,
            y: 0,
            restore_cursor: false,
            width: None,
            height: None,
            truecolor: true,
            use_kitty: true,
            use_iterm: true,
            image_buffer: PhantomData,
        }
    }
}

impl<T> Clone for ViuerPrinter<T> {
    fn clone(&self) -> Self {
        Self {
            transparent: self.transparent.clone(),
            absolute_offset: self.absolute_offset.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            restore_cursor: self.restore_cursor.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            truecolor: self.truecolor.clone(),
            use_kitty: self.use_kitty.clone(),
            use_iterm: self.use_iterm.clone(),
            image_buffer: PhantomData,
        }
    }
}

impl<T> Copy for ViuerPrinter<T> {}

impl<T, F> Fmap<F> for ViuerPrinter<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> IntoMonad for ViuerPrinter<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for ViuerPrinter<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for ViuerPrinter<T> {
    type Inputs = T;
    type Moves = T;
}

impl<T, D> EvaluateFunction<D> for ViuerPrinter<T> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<T> Closure<T> for ViuerPrinter<T>
where
    T: Into<DynamicImage>,
{
    type Output = ();

    fn call(self, input: T) -> Self::Output {
        viuer::print(
            &input.into().to_rgba8().into(),
            &Config {
                transparent: self.transparent,
                absolute_offset: self.absolute_offset,
                x: self.x,
                y: self.y,
                restore_cursor: self.restore_cursor,
                width: self.width,
                height: self.height,
                truecolor: self.truecolor,
                use_kitty: self.use_kitty,
                use_iterm: self.use_iterm,
            },
        )
        .unwrap();
    }
}
