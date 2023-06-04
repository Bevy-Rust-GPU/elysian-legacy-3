use t_funk::macros::functions;

#[functions]
pub trait LiftParam {
    type LiftParam;

    fn lift_param(self) -> Self::LiftParam;
}

pub type LiftParamT<T> = <T as LiftParam>::LiftParam;
