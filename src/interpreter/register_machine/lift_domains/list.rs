use crate::{
    interpreter::register_machine::domain_function::MovesT, DomainFunction, LiftDomainFunctionF,
    LiftDomains, Pair,
};

use t_funk::{
    closure::{CallF, Closure, Compose, ComposeLF, ComposeLT, OutputT},
    collection::set::DropF,
    function::Id,
    macros::types,
    typeclass::arrow::{Fanout, FanoutF, FanoutT, Second, SecondT},
};

impl<D, N, T> LiftDomains<T> for (D, N)
where
    Self: LiftDomainsList<T> + FanoutSetters + ComposeSetters + ComposeRemoves<T>,
    LiftDomainListT<Self, T>: Closure<T>,
    FanoutSettersT<Self>: Closure<OutputT<LiftDomainListT<Self, T>, T>>,
{
    type LiftDomains = ComposeLT<
        FanoutT<
            ComposeLT<
                OutputT<FanoutSettersT<Self>, OutputT<LiftDomainListT<Self, T>, T>>,
                ComposeSettersT<Self>,
            >,
            ComposeRemovesT<Self, T>,
        >,
        CallF,
    >;

    fn lift_domains(input: T) -> Self::LiftDomains {
        Self::lift_domain_list()
            .compose_l(Self::fanout_setters())
            .call(input)
            .compose_l(Self::compose_setters())
            .fanout(Self::compose_removes())
            .compose_l(CallF)
    }
}

/// Given a list of domain types and an ADT implementing those domains,
/// produce a fanout structure of context-lifted domain functions
#[types]
pub trait LiftDomainsList<T> {
    type LiftDomainList;

    fn lift_domain_list() -> Self::LiftDomainList;
}

impl<D, N, T> LiftDomainsList<T> for (D, N)
where
    T: DomainFunction<D>,
    N: LiftDomainsList<T>,
    LiftDomainFunctionF<D>: Fanout<LiftDomainListT<N, T>>,
    N: Pair,
{
    type LiftDomainList = FanoutT<LiftDomainFunctionF<D>, LiftDomainListT<N, T>>;

    fn lift_domain_list() -> Self::LiftDomainList {
        LiftDomainFunctionF::<D>::default().fanout(<N as LiftDomainsList<T>>::lift_domain_list())
    }
}

impl<D, T> LiftDomainsList<T> for (D, ())
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
#[types]
pub trait FanoutSetters {
    type FanoutSetters;

    fn fanout_setters() -> Self::FanoutSetters;
}

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
#[types]
pub trait ComposeSetters {
    type ComposeSetters;

    fn compose_setters() -> Self::ComposeSetters;
}

impl<D, N, N2> ComposeSetters for (D, (N, N2))
where
    (N, N2): ComposeSetters,
    N2: Pair,
{
    type ComposeSetters = ComposeLT<SecondT<ComposeLF>, ComposeSettersT<(N, N2)>>;

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

#[types]
pub trait ComposeRemoves<T> {
    type ComposeRemoves;

    fn compose_removes() -> Self::ComposeRemoves;
}

impl<D, N, T> ComposeRemoves<T> for (D, N)
where
    T: DomainFunction<D>,
    N: Pair + ComposeRemoves<T>,
{
    type ComposeRemoves = ComposeLT<DropF<MovesT<T, D>>, ComposeRemovesT<N, T>>;

    fn compose_removes() -> Self::ComposeRemoves {
        DropF::<MovesT<T, D>>::default().compose_l(N::compose_removes())
    }
}

impl<D, T> ComposeRemoves<T> for (D, ())
where
    T: DomainFunction<D>,
{
    type ComposeRemoves = DropF<MovesT<T, D>>;

    fn compose_removes() -> Self::ComposeRemoves {
        DropF::<MovesT<T, D>>::default()
    }
}
