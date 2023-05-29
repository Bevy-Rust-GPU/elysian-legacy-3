pub struct Identity;

#[macro_export]
macro_rules! impl_identity {
    ($ident:ident $(<$gen:ident>)?) => {
        impl$(<$gen>)? crate::Domain<crate::Identity> for $ident $(<$gen>)? {
            type Domain = type_fields::t_funk::function::Id;

            fn domain(self) -> Self::Domain {
                type_fields::t_funk::function::Id
            }
        }
    };
}
