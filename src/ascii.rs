use std::str::FromStr;

use proc_macro::{TokenStream, TokenTree};

fn emit_compile_error(err: &str) -> TokenStream {
    TokenStream::from_str(&format!("compile_error!(\"{}\")", err)).unwrap()
}

fn is_string_literal(lit: &str) -> bool {
    lit.starts_with('"') && lit.ends_with('"')
}

/// Transforms a string literal such that all ASCII alphabetical characters are in their lowercase form
/// 
/// ```
/// let a = ascii_lowercase!("ABC");
/// assert_eq!(a, "abc");
/// ```
#[proc_macro]
pub fn ascii_lowercase(stream: TokenStream) -> TokenStream {
    let mut token_iter = stream.into_iter();

    if let Some(TokenTree::Literal(lit)) = token_iter.next() {
        if token_iter.next().is_some() {
            return emit_compile_error("Expected argument")
        }

        let lit_str = lit.to_string();

        if !is_string_literal(&lit_str) {
            return emit_compile_error("Provided literal is not a string");
        }

        TokenStream::from_str(&lit_str.to_ascii_lowercase()).unwrap()
    } else {
        emit_compile_error("Expected a string literal")
    }
}

/// Transforms a string literal such that all ASCII alphabetical characters are in their uppercase form
/// 
/// ```
/// let a = ascii_uppercase!("abc");
/// assert_eq!(a, "ABC");
/// ```
#[proc_macro]
pub fn ascii_uppercase(stream: TokenStream) -> TokenStream {
    let mut token_iter = stream.into_iter();

    if let Some(TokenTree::Literal(lit)) = token_iter.next() {
        if token_iter.next().is_some() {
            return emit_compile_error("Expected exactly one argument")
        }

        let lit_str = lit.to_string();

        if !is_string_literal(&lit_str) {
            return emit_compile_error("Provided literal is not a string");
        }

        TokenStream::from_str(&lit_str.to_ascii_uppercase()).unwrap()
    } else {
        emit_compile_error("Expected a string literal")
    }
}