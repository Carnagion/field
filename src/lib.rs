//! Statically-verified `struct` field names as strings.
//!
//! The [`field!`] macro evaluates to a field's name after verifying that the field exists on the specified `struct`.
//! The type must be in scope, and the field must be visible (accessible) at the point of invocation of the macro.
//!
//! # Usage
//!
//! Invoke the macro as `field!(field @ Struct)`:
//! ```
//! use field::*;
//!
//! struct User {
//!     name: String,
//!     age: u32,
//! }
//!
//! let name = field!(name @ User);
//! assert_eq!(name, "name");
//!
//! let age = field!(age @ User);
//! assert_eq!(age, "age");
//! ```
//!
//! A field that is not on the specified `struct` or a type that is not in scope will cause a compilation error:
//! ```compile_fail
//! // This fails because there is no field named "address" on "User"
//! let address = field!(address @ User);
//!
//! // This fails because their is no struct named "NonExistent"
//! let foo = field!(whatever @ NonExistent);
//! ```
//!
//! # Generics
//!
//! [`field!`] also works with generic types, as long as concrete type parameters are provided:
//! ```
//! use field::*;
//!
//! struct Pair<T> {
//!     first: T,
//!     second: T,
//! }
//!
//! let first = field!(first @ Pair<()>);
//! assert_eq!(first, "first");
//!
//! // Any type can be used for the type parameter(s)
//! let second = field!(second @ Pair<i32>);
//! assert_eq!(second, "second");
//! ```
//!
//! # Paths
//!
//! That's right, [`field!`] also works with path syntax:
//! ```
//! use field::*;
//!
//! mod wrap {
//!     pub struct Wrap<T> {
//!         pub inner: T, // Must be pub so that it is visible at the point of invocation
//!     }
//! }
//!
//! let inner = field!(inner @ wrap::Wrap<()>);
//! assert_eq!(inner, "inner");
//! ```
//!
//! # Visibility
//!
//! The specified field and `struct` must be visible (aka accessible) at the point of invocation of the macro:
//! ```
//! use field::*;
//!
//! mod int {
//!     pub struct Int {
//!         pub int: i32,
//!     }
//! }
//!
//! let int = field!(int @ int::Int);
//! assert_eq!(int, "int");
//! ```
//!
//! A type or field that is not visible will result in a compilation error:
//! ```compile_fail
//! use field::*;
//!
//! mod example {
//!     struct Secret {
//!         pub secret: String,
//!     }
//!
//!     pub struct Empty {
//!         empty: ()
//!     }
//! }
//!
//! // This fails because the "Secret" struct is not visible to the outer module
//! let secret = field!(secret @ example::Secret);
//!
//! // This fails because the "empty" field is not visible to the outer module
//! let empty = field!(empty @ example::Empty);
//! ```
//!
//! # Dependencies
//!
//! This crate is completely dependency-free.
//! `#[no_std]` is also supported by default.

#![warn(missing_docs)]
#![no_std]

/// Checks for the presence of a field on a `struct` at compile-time and returns the field's name as a `&'static str`.
///
/// Invoked as `field!(field @ Struct)`, this macro verifies that a field named `field` exists on the type `Struct`, and is visible at the point of invocation.
/// It then stringifies the field name, evaluating to `"field"`.
///
/// Refer to the [crate-level documentation](https://docs.rs/field) for more information.
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

#[cfg(test)]
mod tests {
    #[test]
    fn normal() {
        struct Int {
            int: i32,
        }

        let int = field!(int @ Int);
        assert_eq!(int, "int");
    }

    #[test]
    fn generic() {
        struct Wrap<T> {
            inner: T,
        }

        let inner = field!(inner @ Wrap<()>);
        assert_eq!(inner, "inner");
    }

    // #[test]
    // fn fails() {
    //     struct Foo {
    //         bar: usize,
    //     }
    //     let _ = field!(baz @ Foo);
    // }
}
