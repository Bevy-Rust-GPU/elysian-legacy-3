pub trait LiftModifier {
    type LiftModifier;

    fn lift_modifier(self) -> Self::LiftModifier;
}

pub type LiftModifierT<T> = <T as LiftModifier>::LiftModifier;
