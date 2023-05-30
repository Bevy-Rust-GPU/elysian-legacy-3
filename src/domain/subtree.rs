//! Subtree domain
//!
//! Used to drill down into the leaf element of a shape tree

use crate::{Domain, DomainF};

pub enum Subtree {}

pub type SubtreeT<T> = <T as Domain<Subtree>>::Domain;
pub type SubtreeF = DomainF<Subtree>;

#[macro_export]
macro_rules! impl_subtree {
    ($ty:ident $(<$gen:ident>)?) => {
        impl$(<$gen>)? Domain<crate::Subtree> for $ty $(<$gen>)? {
            type Input = $ty $(<$gen>)?;
            type Domain = type_fields::t_funk::Curry2B<type_fields::t_funk::hlist::PushBackF, $ty $(<$gen>)?>;

            fn domain(self) -> Self::Domain {
                type_fields::t_funk::Curry2::suffix2(type_fields::t_funk::hlist::PushBackF, self)
            }
        }
    };
}
