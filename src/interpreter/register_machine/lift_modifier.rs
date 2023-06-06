use t_funk::macros::types;

#[types]
pub trait LiftModifier {
    type LiftModifier;

    fn lift_modifier(self) -> Self::LiftModifier;
}
