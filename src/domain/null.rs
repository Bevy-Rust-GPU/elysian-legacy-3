#[macro_export]
macro_rules! impl_null {
    ($ident:ident $(<$gen:ident>)?) => {
        impl$(<$gen>)? crate::Domain<()> for $ident $(<$gen>)? {
            type Input = ();
            type Domain = type_fields::t_funk::Curry2A<type_fields::t_funk::function::Const, ()>;

            fn domain(self) -> Self::Domain {
                type_fields::t_funk::Curry2::prefix2(type_fields::t_funk::function::Const, ())
            }
        }
    };
}

