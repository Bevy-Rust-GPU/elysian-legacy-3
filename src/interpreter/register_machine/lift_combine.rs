use t_funk::macros::types;

#[types]
pub trait LiftCombine<D> {
    type LiftCombine;

    fn lift_combine(self) -> Self::LiftCombine;
}
