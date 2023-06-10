use t_funk::macros::functions;

/// A symbol type that can produce a function corresponding to a `Domain<T>`
#[functions]
pub trait DomainFunction<T> {
    type Inputs;
    type Function;

    fn domain(self) -> Self::Function;
}

pub type FunctionT<T, D> = <T as DomainFunction<D>>::Function;
pub type InputsT<T, D> = <T as DomainFunction<D>>::Inputs;
