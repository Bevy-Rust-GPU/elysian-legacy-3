use core::marker::PhantomData;

use t_funk::{
    closure::{Closure, ComposeLF, OutputT},
    collection::map::{Get as GetM, GetT as GetMT, Insert as InsertM, InsertT as InsertMT},
    function::Id,
    typeclass::{
        foldable::{Foldr, FoldrT},
        functor::{Fmap, FmapT},
        monad::Identity,
        semigroup::MappendT,
    },
};

use crate::{
    Alias, BinaryConditional, ContextA, ContextB, ContextOut, CopyContext, EvaluateFunction,
    ExpandAlias, ExpandAliasT, IntoMonad, IntoTupleT, LiftAdt, LiftEvaluateF, Pair, Run, ShapeA,
    ShapeB,
};

use super::{ContextIn, InheritedA, InheritedB};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Left;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Right;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inherited;

// Evaluate side S with domains I (or Inherited), and store output in O
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluateSide<S, I, O>(pub PhantomData<(S, I, O)>);

impl<S, I, O> LiftAdt for EvaluateSide<S, I, O> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<S, I, O, D> EvaluateFunction<D> for EvaluateSide<S, I, O> {
    type Function = Self;

    fn evaluate_function(self) -> Self::Function {
        self
    }
}

impl<O, C> Closure<C> for EvaluateSide<Left, Inherited, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<InheritedA>
        + InsertM<O, OutputT<GetMT<C, InheritedA>, GetMT<C, ContextIn>>>,
    GetMT<C, InheritedA>: Closure<GetMT<C, ContextIn>>,
{
    type Output = InsertMT<C, O, OutputT<GetMT<C, InheritedA>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let inherited_a = GetM::<InheritedA>::get(ctx.clone());
        ctx.insert(inherited_a.call(context_in))
    }
}

impl<D, O, C> Closure<C> for EvaluateSide<Left, D, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<ShapeA>
        + InsertM<
            O,
            OutputT<
                FoldrT<FmapT<GetMT<C, ShapeA>, LiftEvaluateF<D>>, ComposeLF, Id>,
                GetMT<C, ContextIn>,
            >,
        >,
    GetMT<C, ShapeA>: Fmap<LiftEvaluateF<D>>,
    FmapT<GetMT<C, ShapeA>, LiftEvaluateF<D>>: Foldr<ComposeLF, Id>,
    FoldrT<FmapT<GetMT<C, ShapeA>, LiftEvaluateF<D>>, ComposeLF, Id>: Closure<GetMT<C, ContextIn>>,
    D: Pair,
{
    type Output = InsertMT<
        C,
        O,
        OutputT<
            FoldrT<FmapT<GetMT<C, ShapeA>, LiftEvaluateF<D>>, ComposeLF, Id>,
            GetMT<C, ContextIn>,
        >,
    >;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let shape_a = GetM::<ShapeA>::get(ctx.clone());
        ctx.insert(
            shape_a
                .fmap(LiftEvaluateF::<D>::default())
                .foldr(ComposeLF, Id)
                .call(context_in),
        )
    }
}

impl<O, C> Closure<C> for EvaluateSide<Right, Inherited, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<InheritedB>
        + InsertM<O, OutputT<GetMT<C, InheritedB>, GetMT<C, ContextIn>>>,
    GetMT<C, InheritedB>: Closure<GetMT<C, ContextIn>>,
{
    type Output = InsertMT<C, O, OutputT<GetMT<C, InheritedB>, GetMT<C, ContextIn>>>;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let inherited_b = GetM::<InheritedB>::get(ctx.clone());
        ctx.insert(inherited_b.call(context_in))
    }
}

impl<D, O, C> Closure<C> for EvaluateSide<Right, D, O>
where
    C: Clone
        + GetM<ContextIn>
        + GetM<ShapeB>
        + InsertM<
            O,
            OutputT<
                FoldrT<FmapT<GetMT<C, ShapeB>, LiftEvaluateF<D>>, ComposeLF, Id>,
                GetMT<C, ContextIn>,
            >,
        >,
    GetMT<C, ShapeB>: Fmap<LiftEvaluateF<D>>,
    FmapT<GetMT<C, ShapeB>, LiftEvaluateF<D>>: Foldr<ComposeLF, Id>,
    FoldrT<FmapT<GetMT<C, ShapeB>, LiftEvaluateF<D>>, ComposeLF, Id>: Closure<GetMT<C, ContextIn>>,
    D: Pair,
{
    type Output = InsertMT<
        C,
        O,
        OutputT<
            FoldrT<FmapT<GetMT<C, ShapeB>, LiftEvaluateF<D>>, ComposeLF, Id>,
            GetMT<C, ContextIn>,
        >,
    >;

    fn call(self, ctx: C) -> Self::Output {
        let context_in = GetM::<ContextIn>::get(ctx.clone());
        let shape_b = GetM::<ShapeB>::get(ctx.clone());
        ctx.insert(
            shape_b
                .fmap(LiftEvaluateF::<D>::default())
                .foldr(ComposeLF, Id)
                .call(context_in),
        )
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluateBoth<T>(PhantomData<T>);

impl<T> IntoMonad for EvaluateBoth<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for EvaluateBoth<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for EvaluateBoth<T>
where
    T: Default,
{
    type ExpandAlias = (
        EvaluateSide<Left, T, ContextA>,
        EvaluateSide<Right, T, ContextB>,
    );

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluateSelect<D, T, F>(pub F, pub PhantomData<(D, T)>);

impl<D, T, F> IntoMonad for EvaluateSelect<D, T, F> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<D, T, F> LiftAdt for EvaluateSelect<D, T, F> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D, T, F, E> ExpandAlias<E> for EvaluateSelect<D, T, F>
where
    D: Default,
    T: Default,
    F: Default,
{
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<D>, E>,
        IntoTupleT<
            BinaryConditional<
                T,
                F,
                CopyContext<ContextA, ContextOut>,
                CopyContext<ContextB, ContextOut>,
            >,
        >,
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluatePredicated<P, D, T, F>(pub F, pub PhantomData<(P, D, T)>);

impl<P, D, T, F> IntoMonad for EvaluatePredicated<P, D, T, F> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<P, D, T, F> LiftAdt for EvaluatePredicated<P, D, T, F> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<P, D, T, F, E> ExpandAlias<E> for EvaluatePredicated<P, D, T, F>
where
    P: Default,
    D: Default,
    T: Default,
    F: Default,
{
    type ExpandAlias = MappendT<
        ExpandAliasT<EvaluateBoth<P>, E>,
        IntoTupleT<
            BinaryConditional<
                T,
                F,
                EvaluateSide<Left, D, ContextOut>,
                EvaluateSide<Right, D, ContextOut>,
            >,
        >,
    >;

    fn expand_alias(self) -> Self::ExpandAlias {
        Default::default()
    }
}
