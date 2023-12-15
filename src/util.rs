use t_funk::macros::lift;

/// Private trait identifying types equivalent to a 2-tuple
/// Used to make recursive implementations distinct from their tail case
/// i.e. (A, B) asserts that B is a Pair,
/// so (A, ()) can be implemented without conflict as () is not a Pair
pub(crate) trait Pair {}

impl<A, B> Pair for (A, B) {}

/// Clamp a float to the 0.0..=1.0 range
#[lift]
pub fn saturate(t: f32) -> f32 {
    t.max(0.0).min(1.0)
}

/// Invert within the 0.0..=1.0 range
#[lift]
pub fn invert(t: f32) -> f32 {
    1.0 - t
}

/// Index into a collection
#[lift]
pub fn index<T, I>(t: T, i: I) -> IndexT<T, I>
where
    T: core::ops::Index<I>,
    T::Output: Clone,
{
    t.index(i).clone()
}

pub type IndexT<T, I> = <T as core::ops::Index<I>>::Output;

/// Cast a float to a usize
#[lift]
pub fn as_usize(t: f32) -> usize {
    t as usize
}

#[cfg(feature = "std")]
mod standard {
    use image::{DynamicImage, RgbaImage};
    use t_funk::macros::lift;

    /// Convert a collection of characters into a string
    #[lift]
    pub fn chars_to_string<T>(t: T) -> String
    where
        T: IntoIterator<Item = char>,
    {
        t.into_iter().map(|c| c.to_string()).collect()
    }

    /// Convert a DynamicImage into an RgbaImage
    #[lift]
    pub fn to_rgba8(t: DynamicImage) -> RgbaImage {
        t.to_rgba8()
    }
}

#[cfg(feature = "std")]
pub use standard::*;
