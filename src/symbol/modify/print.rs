use t_funk::{function::PrintLn, typeclass::functor::Fmap};

use crate::{EvaluateFunction, LiftAdt, Evaluable, LiftModify, Run};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Print;

impl<F> Fmap<F> for Print {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for Print {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl Evaluable for Print {
    type Lift = LiftModify;
}

impl<D> EvaluateFunction<D> for Print {
    type Inputs = String;
    type Moves = ();
    type Function = PrintLn;

    fn evaluate_function(self) -> Self::Function {
        PrintLn
    }
}
