#[macro_export]
macro_rules! field {
    ($field:ident @ $structure:ty) => {{
        #[allow(unused)]
        fn assert_field_exists(on: $structure) {
            let _ = on.$field;
        }
        ::core::stringify!($field)
    }};
}
