mod list;

use std::marker::PhantomData;

pub use list::*;
use t_funk::{function::Function, macros::Closure};

/// Given a structure of shape subtypes, and a list of domains,
/// produce a function that takes a context, reads input from it,
/// evaluates the relevant domain functions, and produces an updated context
pub trait LiftDomains<T> {
    type LiftDomains;

    fn lift_domains(input: T) -> Self::LiftDomains;
}

pub type LiftDomainsT<D, T> = <D as LiftDomains<T>>::LiftDomains;

#[derive(Closure)]
pub struct LiftDomainsF<T>(PhantomData<T>);

impl<T> Default for LiftDomainsF<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for LiftDomainsF<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for LiftDomainsF<T> {}

impl<D, T> Function<T> for LiftDomainsF<D>
where
    D: LiftDomains<T>,
{
    type Output = LiftDomainsT<D, T>;

    fn call(input: T) -> Self::Output {
        D::lift_domains(input)
    }
}
