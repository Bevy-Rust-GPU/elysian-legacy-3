use image::DynamicImage;
use t_funk::{
    closure::{Compose, Composed, Curry2, Curry2A},
    function::IntoF,
    macros::lift,
};
use viuer::{Config, ViuResult};

use crate::{Image, Rasterize, ToRgba8};

#[lift]
pub fn viuer_print(config: Config, image: DynamicImage) -> ViuResult<(u32, u32)> {
    viuer::print(&image, &config)
}

pub type Viuer<D, C, F> = Composed<
    Curry2A<ViuerPrint, Config>,
    Composed<
        IntoF<DynamicImage>,
        Composed<ToRgba8, Composed<IntoF<DynamicImage>, Composed<Image<C, F>, Rasterize<D>>>>,
    >,
>;

pub fn make_viuer_raw<D, C, F>(
    width: usize,
    height: usize,
    config: Config,
) -> Viuer<D, C, F> {
    Rasterize {
        width,
        height,
        ..Default::default()
    }
    .compose_l(Image::<C, F>::default())
    .compose_l(IntoF::<DynamicImage>::default())
    .compose_l(ToRgba8) // Convert to RGBA8 for compatibility
    .compose_l(IntoF::<DynamicImage>::default())
    .compose_l(ViuerPrint.prefix2(config))
}

pub fn make_viuer<D, C, F>(w: usize, h: usize) -> Viuer<D, C, F> {
    make_viuer_raw::<D, C, F>(
        w,
        h,
        Config {
            absolute_offset: false,
            width: Some(w as u32),
            ..Default::default()
        },
    )
}
