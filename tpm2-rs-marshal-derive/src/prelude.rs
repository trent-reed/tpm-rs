pub use proc_macro2::{Span, TokenStream};
pub use quote::{quote, quote_spanned, ToTokens};
pub use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Error, Expr, ExprPath, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, LitInt,
    MetaNameValue, Result, Type, Variant,
};
