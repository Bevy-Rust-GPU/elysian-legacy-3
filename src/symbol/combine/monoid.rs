use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{
        foldable::{FoldMap, FoldMapT},
        functor::Fmap,
        monad::Chain,
        monoid::{Mconcat, Mempty},
        semigroup::Mappend,
    },
};

use crate::{Combine, IntoMonad, IntoTuple, IntoTupleT};

pub trait FoldCombine<F> {
    type FoldCombine;

    fn fold_combine(self, f: F) -> Self::FoldCombine;
}

impl<T, F> FoldCombine<F> for T
where
    T: FoldMap<MakeMonoidCombine<F>>,
{
    type FoldCombine = FoldMapT<T, MakeMonoidCombine<F>>;

    fn fold_combine(self, f: F) -> Self::FoldCombine {
        self.fold_map(MakeMonoidCombine(f))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MakeMonoidCombine<F>(pub F);

impl<F, T> Closure<T> for MakeMonoidCombine<F> {
    type Output = MonoidCombine<T, F>;

    fn call(self, input: T) -> Self::Output {
        MonoidCombine(input, self.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonoidCombine<T, F>(T, F);

impl<T, F, G> Fmap<G> for MonoidCombine<T, F>
where
    G: Closure<T>,
{
    type Fmap = MonoidCombine<OutputT<G, T>, F>;

    fn fmap(self, f: G) -> Self::Fmap {
        MonoidCombine(f.call(self.0), self.1)
    }
}

impl<T, F, G> Chain<G> for MonoidCombine<T, F>
where
    G: Closure<T>,
{
    type Chain = OutputT<G, T>;

    fn chain(self, f: G) -> Self::Chain {
        f.call(self.0)
    }
}

impl<T, F> IntoMonad for MonoidCombine<T, F> {
    type IntoMonad = Self;

    fn into_monad(self) -> Self::IntoMonad {
        self
    }
}

impl<T, F> Mempty for MonoidCombine<T, F>
where
    F: Default,
{
    type Mempty = ();

    fn mempty() -> Self::Mempty {
        ()
    }
}

impl<T, F, U> Mappend<MonoidCombine<U, F>> for MonoidCombine<T, F>
where
    T: IntoTuple,
    U: IntoTuple,
    F: Clone + IntoTuple,
{
    type Mappend = MonoidCombine<Combine<IntoTupleT<T>, IntoTupleT<U>, IntoTupleT<F>>, F>;

    fn mappend(self, t: MonoidCombine<U, F>) -> Self::Mappend {
        MonoidCombine(
            Combine(
                self.0.into_tuple(),
                t.0.into_tuple(),
                self.1.clone().into_tuple(),
            ),
            self.1,
        )
    }
}

impl<T, F> Mappend<MonoidCombine<T, F>> for () {
    type Mappend = MonoidCombine<T, F>;

    fn mappend(self, t: MonoidCombine<T, F>) -> Self::Mappend {
        t
    }
}

impl<T, F> Mappend<()> for MonoidCombine<T, F> {
    type Mappend = Self;

    fn mappend(self, (): ()) -> Self::Mappend {
        self
    }
}

impl<T, F> Mconcat for MonoidCombine<T, F> {
    type Mconcat = Self;

    fn mconcat(self) -> Self::Mconcat {
        self
    }
}

