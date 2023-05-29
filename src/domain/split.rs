use std::marker::PhantomData;

use crate::{Domain, DomainF};

pub struct Split<A, B>(PhantomData<(A, B)>);

pub type SplitT<T, A, B> = <T as Domain<Split<A, B>>>::Domain;
pub type SplitF<A, B> = DomainF<Split<A, B>>;

#[macro_export]
macro_rules! impl_split {
    ($ty:ident $(<$gen:ident>)?) => {
        impl<$($gen,)? A, B> Domain<crate::Split<A, B>> for $ty $(<$gen>)?
        where
            $ty $(<$gen>)?: Clone + crate::Domain<A> + crate::Domain<B>,
            crate::DomainT<$ty $(<$gen>)?, A>: type_fields::t_funk::arrow::Split<crate::DomainT<$ty $(<$gen>)?, B>>,
        {
            type Domain =
                type_fields::t_funk::arrow::SplitT<crate::DomainT<$ty $(<$gen>)?, A>, crate::DomainT<$ty $(<$gen>)?, B>>;

            fn domain(self) -> Self::Domain {
                type_fields::t_funk::Split::split(
                    crate::Domain::<A>::domain(self.clone()),
                    crate::Domain::<B>::domain(self.clone()),
                )
            }
        }
    };
}
