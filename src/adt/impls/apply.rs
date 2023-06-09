use t_funk::typeclass::{
    applicative::{Apply, ApplyT},
    functor::{Fmap, FmapT},
    monoid::{Mempty, MemptyT},
    semigroup::{Mappend, MappendT},
};

use crate::{AdtEnd, Combine, Run, Then};

impl<A, T> Apply<T> for Run<A>
where
    T: Fmap<A>,
{
    type Apply = FmapT<T, A>;

    fn apply(self, t: T) -> Self::Apply {
        t.fmap(self.0)
    }
}

impl<A, B, T> Apply<T> for Then<A, B>
where
    A: Apply<T>,
    B: Apply<T>,
    T: Clone,
    ApplyT<A, T>: Mappend<ApplyT<B, T>>,
{
    type Apply = MappendT<ApplyT<A, T>, ApplyT<B, T>>;

    fn apply(self, t: T) -> Self::Apply {
        self.0.apply(t.clone()).mappend(self.1.apply(t))
    }
}

impl<A, B, F, T> Apply<T> for Combine<A, B, F>
where
    A: Apply<T>,
    B: Apply<T>,
    T: Clone,
    ApplyT<A, T>: Mappend<ApplyT<B, T>>,
{
    type Apply = MappendT<ApplyT<A, T>, ApplyT<B, T>>;

    fn apply(self, t: T) -> Self::Apply {
        self.0.apply(t.clone()).mappend(self.1.apply(t))
    }
}

impl<T> Apply<T> for AdtEnd
where
    T: Mempty,
{
    type Apply = MemptyT<T>;

    fn apply(self, _: T) -> Self::Apply {
        T::mempty()
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;
    use t_funk::{
        closure::{Const, Curry2},
        collection::hlist::{Cons, Nil},
        function::Mul,
        op_chain::Done,
        typeclass::{applicative::Apply, functor::Fmap},
    };

    use crate::{adt, ContextGet, Distance, Isosurface, Point, Translate};

    #[test]
    fn test_adt_apply() {
        let shape = adt() << Translate(Vec2::new(0.5, 0.5)) << Point << Isosurface(0.2_f32)
            >> adt()
            << ContextGet::<Distance<f32>>::default()
            >> Done;

        let funcs = shape.fmap(Const(Mul.suffix2(2)));
        let vals = funcs.apply(Cons(2, Cons(3, Nil)));
        assert_eq!(vals, Cons(4, Cons(6, Cons(4, Cons(6, Nil)))));
    }
}
