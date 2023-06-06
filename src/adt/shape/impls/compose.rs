use t_funk::typeclass::category::{Compose, ComposeT};

use crate::{Field, Input, Nil, NotNil, Output};

// Non-terminating Input defers its inner type
impl<A, B, C> Compose<C> for Input<A, B>
where
    B: NotNil + Compose<C>,
{
    type Compose = Input<A, ComposeT<B, C>>;

    fn compose(self, rhs: C) -> Self::Compose {
        Input(self.0, self.1.compose(rhs))
    }
}

// Terminating inputs can compose Input or Field
impl<A, B, C> Compose<Input<B, C>> for Input<A, Nil> {
    type Compose = Input<A, Input<B, C>>;

    fn compose(self, rhs: Input<B, C>) -> Self::Compose {
        Input(self.0, rhs)
    }
}

impl<A, B, C> Compose<Field<B, C>> for Input<A, Nil> {
    type Compose = Input<A, Field<B, C>>;

    fn compose(self, rhs: Field<B, C>) -> Self::Compose {
        Input(self.0, rhs)
    }
}

// Non-terminating Field defers to its inner type
impl<A, B, C> Compose<C> for Field<A, B>
where
    B: NotNil + Compose<C>,
{
    type Compose = Field<A, ComposeT<B, C>>;

    fn compose(self, rhs: C) -> Self::Compose {
        Field(self.0, self.1.compose(rhs))
    }
}

// Terminating Field can compose an Output
impl<A, B, C> Compose<Output<B, C>> for Field<A, Nil> {
    type Compose = Field<A, Output<B, C>>;

    fn compose(self, rhs: Output<B, C>) -> Self::Compose {
        Field(self.0, rhs)
    }
}

// Non-terminating Output defers to its inner type
impl<A, B, C> Compose<C> for Output<A, B>
where
    B: NotNil + Compose<C>,
{
    type Compose = Output<A, ComposeT<B, C>>;

    fn compose(self, rhs: C) -> Self::Compose {
        Output(self.0, self.1.compose(rhs))
    }
}

// Terminating Output can compose an Output
impl<A, B, C> Compose<Output<B, C>> for Output<A, Nil> {
    type Compose = Output<A, Output<B, C>>;

    fn compose(self, rhs: Output<B, C>) -> Self::Compose {
        Output(self.0, rhs)
    }
}

impl<T> Compose<T> for Nil {
    type Compose = T;

    fn compose(self, f: T) -> Self::Compose {
        f
    }
}

#[cfg(test)]
mod test {
    use t_funk::{closure::Const, r#do::Done};

    use crate::{shape, Isosurface, Point, Translate};

    #[test]
    fn test_compose_shape() {
        let input = Translate(Const(0.0), Const(0.0));
        let field = Point;
        let output = Isosurface(Const(0.0));
        let _shape = shape() << input << field << output >> Done;
    }
}
