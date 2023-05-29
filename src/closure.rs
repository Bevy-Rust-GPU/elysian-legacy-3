use image::{DynamicImage, RgbaImage};
use type_fields::{macros::Closure, t_funk::{Function, Min, Closure, Max}};

/// Clamp a float to the 0.0..=1.0 range
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Saturate;

impl Function<f32> for Saturate {
    type Output = f32;

    fn call(input: f32) -> Self::Output {
        Min.call((Max.call((input, 0.0)), 1.0))
    }
}

/// Return one minus the given float
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Invert;

impl Function<f32> for Invert {
    type Output = f32;

    fn call(input: f32) -> Self::Output {
        1.0 - input
    }
}

/// Index into a collection
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct Index;

impl<T, I> Function<(T, I)> for Index
where
    T: std::ops::Index<I>,
    T::Output: Clone,
{
    type Output = T::Output;

    fn call((t, i): (T, I)) -> Self::Output {
        t.index(i).clone()
    }
}

/// Cast a type to a usize
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct AsUsize;

impl Function<f32> for AsUsize {
    type Output = usize;

    fn call(t: f32) -> Self::Output {
        t as usize
    }
}

/// Convert a collection of characters into a string
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct CharsToString;

impl<T> Function<T> for CharsToString
where
    T: IntoIterator<Item = char>,
{
    type Output = String;

    fn call(t: T) -> Self::Output {
        t.into_iter().map(|c| c.to_string()).collect::<String>()
    }
}

/// Convert a DynamicImage into an RgbaImage
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure)]
pub struct ToRgba8;

impl Function<DynamicImage> for ToRgba8 {
    type Output = RgbaImage;

    fn call(input: DynamicImage) -> Self::Output {
        input.to_rgba8()
    }
}

