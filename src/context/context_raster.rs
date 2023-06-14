use t_funk::macros::set::set;

use crate::Raster;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<C>(pub C);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[set]
pub struct ContextRaster<C, R> {
    #[newtype = Context::<C>]
    pub context: C,
    #[newtype = Raster::<R>]
    pub raster: R,
}
