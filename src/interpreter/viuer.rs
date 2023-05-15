use image::{DynamicImage, ImageBuffer, Pixel, RgbaImage};
use type_fields::{
    macros::Closure,
    t_funk::{closure::Compose, Composed, Curry2, Curry2A, Function, IntoF},
};
use viuer::{Config, ViuResult};

use crate::Image;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct ViuerPrint;

impl Function<(Config, DynamicImage)> for ViuerPrint {
    type Output = ViuResult<(u32, u32)>;

    fn call((config, image): (Config, DynamicImage)) -> Self::Output {
        viuer::print(&image, &config)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct ToRgba8;

impl Function<DynamicImage> for ToRgba8 {
    type Output = RgbaImage;

    fn call(input: DynamicImage) -> Self::Output {
        input.to_rgba8()
    }
}

pub type Viuer<D, P, C> = Composed<
    Curry2A<ViuerPrint, Config>,
    Composed<
        IntoF<DynamicImage>,
        Composed<ToRgba8, Composed<IntoF<DynamicImage>, Curry2A<Image<D>, ImageBuffer<P, C>>>>,
    >,
>;

pub fn make_viuer_raw<D, P: Pixel, C>(image: ImageBuffer<P, C>, config: Config) -> Viuer<D, P, C> {
    Image::<D>::default()
        .prefix2(image)
        .compose_l(IntoF::<DynamicImage>::default())
        .compose_l(ToRgba8) // Convert to RGBA8 for compatibility
        .compose_l(IntoF::<DynamicImage>::default())
        .compose_l(ViuerPrint.prefix2(config))
}

pub fn make_viuer<D, P: Pixel>(w: u32, h: u32) -> Viuer<D, P, Vec<P::Subpixel>> {
    make_viuer_raw(
        ImageBuffer::new(w, h),
        Config {
            absolute_offset: false,
            width: Some(w),
            ..Default::default()
        },
    )
}
