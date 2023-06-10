use std::marker::PhantomData;

use image::DynamicImage;
use t_funk::{closure::Closure, typeclass::functor::Fmap};
use viuer::Config;

use crate::{LiftAdt, Modify, ModifyFunction};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            transparent: false,
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

impl<T, F> Fmap<F> for ViuerPrinter<T> {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl<T> LiftAdt for ViuerPrinter<T> {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> ModifyFunction<D> for ViuerPrinter<T> {
    type Inputs = T;

    type Function = ViuerPrintF;

    fn modify_function(self) -> Self::Function {
        ViuerPrintF {
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
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViuerPrintF {
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
}

impl<T> Closure<T> for ViuerPrintF
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

