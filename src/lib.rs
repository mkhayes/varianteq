//! This crate provides a macro to implement equality of enum variants.
//!
//! Two enum variants are equal if they are the same variant from the same enum, regardless of the
//! values of the fields each variant contains.
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate varianteq;
//! #
//! #[derive(VariantEq)]
//! enum Enum {
//!     Variant,
//! }
//! #
//! # fn main() {}
//! ```
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate varianteq;
//!
//! #[derive(Debug, VariantEq)]
//! enum E {
//!     A(i32),
//!     B(i32),
//!     C(u32, bool),
//! }
//!
//! fn main() {
//!     assert_eq!(E::A(1), E::A(2));
//!     assert_ne!(E::A(1), E::B(1));
//!     assert_ne!(E::A(1), E::C(1, false));
//! }
//! ```
//!
//! # Errors
//!
//! The `VariantEq` macro only applies to enums and will cauase a compilation error if used on
//! structs.
//!
//! ```compile_fail
//! # #[macro_use]
//! # extern crate varianteq;
//! #
//! #[derive(VariantEq)]
//! struct S;
//! #
//! # fn main() {}
//! ```
//!
//! ```text
//! error: #[derive(VariantEq)] is only defined for enums
//! ```

use syn::{parse2};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(VariantEq)]
pub fn varianteq_derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse2(tokens.into()).unwrap();
    proc_macro::TokenStream::from(derive(input))
}


fn derive(item: DeriveInput) -> proc_macro2::TokenStream {
    match item.data {
        Data::Enum(_) => (),
        _ => unimplemented!("#[derive(VariantEq)] is only defined for enums"),
    };

    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();

    quote! {
        impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
            fn eq(&self, other: &#ident#ty_generics) -> bool {
                ::std::mem::discriminant(self) == ::std::mem::discriminant(other)
            }
        }
        impl #impl_generics Eq for #ident #ty_generics #where_clause {}
    }
}
