use t_funk::{
    macros::impl_adt,
    typeclass::{
        category::{Compose, ComposeT},
        semigroup::Mappend,
    },
};

use crate::{Combine, LiftAdt, LiftAdtT, Nil, Sequence, Unit};

impl_adt! {
    impl<A, B, C, T> Mappend<T> for Nil | Unit<A> | Sequence<A, B> | Combine<A, B, C>
    where
        Self: Compose<T>,
    {
        type Mappend = ComposeT<Self, T>;

        fn mappend(self, t: T) -> Self::Mappend {
            self.compose(t)
        }
    }
}

#[cfg(test)]
mod test {
    use t_funk::typeclass::semigroup::Mappend;

    use crate::{symbol::Union, Combine, Field, Nil, Point, Sequence, Unit};

    #[test]
    fn test_adt_mappend() {
        let nil = Nil;
        let unit = Unit(Field(Point));
        let sequence = Sequence(unit, Sequence(unit, Nil));
        let combine = Combine(unit, unit, Union);

        assert_eq!(nil.mappend(nil), nil);
        assert_eq!(nil.mappend(unit), unit);
        assert_eq!(nil.mappend(sequence), sequence);
        assert_eq!(nil.mappend(combine), combine);

        assert_eq!(unit.mappend(nil), unit);
        assert_eq!(unit.mappend(unit), sequence);
        assert_eq!(unit.mappend(sequence), Sequence(unit, sequence));
        assert_eq!(
            unit.mappend(combine),
            Sequence(unit, Sequence(combine, Nil))
        );

        assert_eq!(sequence.mappend(nil), sequence);
        assert_eq!(
            sequence.mappend(unit),
            Sequence(
                Unit(Field(Point)),
                Sequence(Unit(Field(Point)), Sequence(unit, Nil))
            )
        );
        assert_eq!(
            sequence.mappend(sequence),
            Sequence(
                Unit(Field(Point)),
                Sequence(
                    Unit(Field(Point)),
                    Sequence(Unit(Field(Point)), Sequence(Unit(Field(Point)), Nil))
                )
            )
        );
        assert_eq!(
            sequence.mappend(combine),
            Sequence(
                Unit(Field(Point)),
                Sequence(
                    Unit(Field(Point)),
                    Sequence(
                        combine,
                        Nil
                    )
                )
            )
        );

        assert_eq!(combine.mappend(nil), combine);
        assert_eq!(
            combine.mappend(unit),
            Sequence(combine, Sequence(unit, Nil))
        );
        assert_eq!(
            combine.mappend(sequence),
            Sequence(combine, Sequence(unit, Sequence(unit, Nil)))
        );
        assert_eq!(combine.mappend(combine), Sequence(combine, combine));
    }
}
