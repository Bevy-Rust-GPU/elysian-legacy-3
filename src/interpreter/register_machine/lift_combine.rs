pub trait LiftCombine {
    type LiftCombine;

    fn lift_combine(self) -> Self::LiftCombine;
}

pub type LiftCombineT<T> = <T as LiftCombine>::LiftCombine;

