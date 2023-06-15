use t_funk::collection::map::{Get, Insert, Remove};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShapeA;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShapeB;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextIn;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextA;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextB;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextOut;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InheritedA;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InheritedB;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    pub shape_a: A,
    pub shape_b: B,
    pub context_in: CI,
    pub context_a: CA,
    pub context_b: CB,
    pub context_out: CO,
    pub inherited_a: FA,
    pub inherited_b: FB,
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ShapeA> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Get = A;

    fn get(self) -> Self::Get {
        self.shape_a
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ShapeB> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Get = B;

    fn get(self) -> Self::Get {
        self.shape_b
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ContextIn> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Get = CI;

    fn get(self) -> Self::Get {
        self.context_in
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ContextA> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Get = CA;

    fn get(self) -> Self::Get {
        self.context_a
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ContextB> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Get = CB;

    fn get(self) -> Self::Get {
        self.context_b
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<ContextOut>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Get = CO;

    fn get(self) -> Self::Get {
        self.context_out
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<InheritedA>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Get = FA;

    fn get(self) -> Self::Get {
        self.inherited_a
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Get<InheritedB>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Get = FB;

    fn get(self) -> Self::Get {
        self.inherited_b
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ShapeA, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<T, B, CI, CA, CB, CO, FA, FB>;

    fn insert(self, shape_a: T) -> Self::Insert {
        let CombineContext {
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ShapeB, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, T, CI, CA, CB, CO, FA, FB>;

    fn insert(self, shape_b: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ContextIn, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, T, CA, CB, CO, FA, FB>;

    fn insert(self, context_in: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ContextA, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, CI, T, CB, CO, FA, FB>;

    fn insert(self, context_a: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ContextB, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, CI, CA, T, CO, FA, FB>;

    fn insert(self, context_b: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_out,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<ContextOut, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, CI, CA, CB, T, FA, FB>;

    fn insert(self, context_out: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            inherited_a,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<InheritedA, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, CI, CA, CB, CO, T, FB>;

    fn insert(self, inherited_a: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_b,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB, T> Insert<InheritedB, T>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Insert = CombineContext<A, B, CI, CA, CB, CO, FA, T>;

    fn insert(self, inherited_b: T) -> Self::Insert {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            ..
        } = self;

        CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        }
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ShapeA> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Removed = CombineContext<(), B, CI, CA, CB, CO, FA, FB>;
    type Remove = A;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a: (),
                shape_b,
                context_in,
                context_a,
                context_b,
                context_out,
                inherited_a,
                inherited_b,
            },
            shape_a,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ShapeB> for CombineContext<A, B, CI, CA, CB, CO, FA, FB> {
    type Removed = CombineContext<A, (), CI, CA, CB, CO, FA, FB>;
    type Remove = B;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b: (),
                context_in,
                context_a,
                context_b,
                context_out,
                inherited_a,
                inherited_b,
            },
            shape_b,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ContextIn>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, (), CA, CB, CO, FA, FB>;
    type Remove = CI;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in: (),
                context_a,
                context_b,
                context_out,
                inherited_a,
                inherited_b,
            },
            context_in,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ContextA>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, CI, (), CB, CO, FA, FB>;
    type Remove = CA;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in,
                context_a: (),
                context_b,
                context_out,
                inherited_a,
                inherited_b,
            },
            context_a,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ContextB>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, CI, CA, (), CO, FA, FB>;
    type Remove = CB;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in,
                context_a,
                context_b: (),
                context_out,
                inherited_a,
                inherited_b,
            },
            context_b,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<ContextOut>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, CI, CA, CB, (), FA, FB>;
    type Remove = CO;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in,
                context_a,
                context_b,
                context_out: (),
                inherited_a,
                inherited_b,
            },
            context_out,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<InheritedA>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, CI, CA, CB, CO, (), FB>;
    type Remove = FA;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in,
                context_a,
                context_b,
                context_out,
                inherited_a: (),
                inherited_b,
            },
            inherited_a,
        )
    }
}

impl<A, B, CI, CA, CB, CO, FA, FB> Remove<InheritedB>
    for CombineContext<A, B, CI, CA, CB, CO, FA, FB>
{
    type Removed = CombineContext<A, B, CI, CA, CB, CO, FA, ()>;
    type Remove = FB;

    fn remove(self) -> (Self::Removed, Self::Remove) {
        let CombineContext {
            shape_a,
            shape_b,
            context_in,
            context_a,
            context_b,
            context_out,
            inherited_a,
            inherited_b,
        } = self;

        (
            CombineContext {
                shape_a,
                shape_b,
                context_in,
                context_a,
                context_b,
                context_out,
                inherited_a,
                inherited_b: (),
            },
            inherited_b,
        )
    }
}

