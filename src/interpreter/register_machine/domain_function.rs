use t_funk::macros::{functions, types};

/// A symbol type that can produce a function corresponding to a `Domain<T>`
#[functions]
#[types]
pub trait DomainFunction<T> {
    type Inputs;
    type Function;

    fn domain(self) -> Self::Function;
}

