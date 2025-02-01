extern crate proc_macro;
use proc_macro::TokenStream;

mod common;
mod css_class;

/// A macro that implements the `CssClass` trait and various other common derives for an enum.
#[proc_macro_attribute]
pub fn css_class(args: TokenStream, input: TokenStream) -> TokenStream {
    css_class::expand(args, input)
}
