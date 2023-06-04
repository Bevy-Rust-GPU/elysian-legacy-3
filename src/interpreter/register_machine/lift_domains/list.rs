use crate::{DomainFunction, LiftDomainFunctionF, LiftDomains, Pair};

use t_funk::{
    closure::{CallF, Closure, Compose, ComposeLF, ComposeLT, OutputT},
    function::Id,
    typeclass::arrow::{Fanout, FanoutF, FanoutT, Second, SecondT},
};

impl<D, N, T> LiftDomains<T> for (D, N)
where
    Self: LiftDomainList<T> + FanoutSetters + ComposeSetters,
    LiftDomainListT<Self, T>: Closure<T>,
    FanoutSettersT<Self>: Closure<OutputT<LiftDomainListT<Self, T>, T>>,
{
    type LiftDomains = ComposeLT<
        FanoutT<
            ComposeLT<
                OutputT<FanoutSettersT<Self>, OutputT<LiftDomainListT<Self, T>, T>>,
                ComposeDomainsT<Self>,
            >,
            Id,
        >,
        CallF,
    >;

    fn lift_domains(input: T) -> Self::LiftDomains {
        Self::lift_domain_list()
            .compose_l(Self::fanout_setters())
            .call(input)
            .compose_l(Self::compose_setters())
            .fanout(Id)
            .compose_l(CallF)
    }
}

/// Given a list of domain types and an ADT implementing those domains,
/// produce a fanout structure of context-lifted domain functions
pub trait LiftDomainList<T> {
    type LiftDomainList;

    fn lift_domain_list() -> Self::LiftDomainList;
}

pub type LiftDomainListT<D, T> = <D as LiftDomainList<T>>::LiftDomainList;

impl<D, N, T> LiftDomainList<T> for (D, N)
where
    T: DomainFunction<D>,
    N: LiftDomainList<T>,
    LiftDomainFunctionF<D>: Fanout<LiftDomainListT<N, T>>,
    N: Pair,
{
    type LiftDomainList = FanoutT<LiftDomainFunctionF<D>, LiftDomainListT<N, T>>;

    fn lift_domain_list() -> Self::LiftDomainList {
        LiftDomainFunctionF::<D>::default().fanout(<N as LiftDomainList<T>>::lift_domain_list())
    }
}

impl<D, T> LiftDomainList<T> for (D, ())
where
    T: DomainFunction<D>,
{
    type LiftDomainList = LiftDomainFunctionF<D>;

    fn lift_domain_list() -> Self::LiftDomainList {
        LiftDomainFunctionF::<D>::default()
    }
}

/// Given a list of domain functions,
/// produce a function that will fanout their resulting setters
pub trait FanoutSetters {
    type FanoutSetters;

    fn fanout_setters() -> Self::FanoutSetters;
}

type FanoutSettersT<T> = <T as FanoutSetters>::FanoutSetters;

impl<D, N, N2> FanoutSetters for (D, (N, N2))
where
    (N, N2): FanoutSetters,
    N2: Pair,
{
    type FanoutSetters = ComposeLT<SecondT<FanoutF>, FanoutSettersT<(N, N2)>>;

    fn fanout_setters() -> Self::FanoutSetters {
        FanoutF.second().compose_l(<(N, N2)>::fanout_setters())
    }
}

impl<D, N> FanoutSetters for (D, (N, ())) {
    type FanoutSetters = FanoutF;

    fn fanout_setters() -> Self::FanoutSetters {
        FanoutF
    }
}

impl<D> FanoutSetters for (D, ()) {
    type FanoutSetters = Id;

    fn fanout_setters() -> Self::FanoutSetters {
        Id
    }
}

/// Given a list of domain functions,
/// produce a function that will compose the fanout structure
/// of their resulting setters
pub trait ComposeSetters {
    type ComposeSetters;

    fn compose_setters() -> Self::ComposeSetters;
}

pub type ComposeDomainsT<T> = <T as ComposeSetters>::ComposeSetters;

impl<D, N, N2> ComposeSetters for (D, (N, N2))
where
    (N, N2): ComposeSetters,
    N2: Pair,
{
    type ComposeSetters = ComposeLT<SecondT<ComposeLF>, ComposeDomainsT<(N, N2)>>;

    fn compose_setters() -> Self::ComposeSetters {
        ComposeLF.second().compose_l(<(N, N2)>::compose_setters())
    }
}

impl<D, N> ComposeSetters for (D, (N, ())) {
    type ComposeSetters = ComposeLF;

    fn compose_setters() -> Self::ComposeSetters {
        ComposeLF
    }
}

impl<D> ComposeSetters for (D, ()) {
    type ComposeSetters = Id;

    fn compose_setters() -> Self::ComposeSetters {
        Id
    }
}
