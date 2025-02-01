extern crate proc_macro;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Fields, Item, ItemEnum,
};

use crate::common;

#[derive(FromMeta, Default)]
#[darling(default)]
struct Options {
    prefix: Option<String>,
    suffix: Option<String>,
}

impl Parse for Options {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let options = if input.is_empty() {
            Default::default()
        } else {
            let meta_list = common::parse_meta_list(&input)?;
            Self::from_list(&meta_list)?
        };

        Ok(options)
    }
}

pub(crate) fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as Options);
    let item = parse_macro_input!(item as Item);

    expand_from_parsed(options, item).into()
}

fn expand_from_parsed(options: Options, mut item: Item) -> TokenStream2 {
    match item {
        Item::Enum(ref mut item) => expand_enum(options, item),
        _ => panic!("cc_class can only be used on enums"),
    }
}

fn expand_enum(options: Options, item: &mut ItemEnum) -> TokenStream2 {
    let mut impl_default = false;
    for variant in &item.variants {
        // Ensure that the enum only has unit variants
        let fields = &variant.fields;
        let is_struct = matches!(fields, Fields::Named(_));
        let is_tuple = matches!(fields, Fields::Unnamed(f) if f.unnamed.len() > 1);
        if is_struct || is_tuple {
            panic!("cc_class can only be used on enums with unit variants");
        }

        // Check if the enum has a default attribute
        variant.attrs.iter().for_each(|attr| {
            if attr.path().is_ident("default") {
                impl_default = true;
            }
        });
    }

    let ident = item.ident.clone();
    let match_arms = item.variants.iter().map(|variant| {
        let variant = &variant.ident;
        // Generate the name with the prefix and suffix
        let prefix = options.prefix.as_deref().unwrap_or("");
        let suffix = options.suffix.as_deref().unwrap_or("");
        let display = variant.to_string().to_lowercase();
        let display = format!("{prefix}{display}{suffix}");
        quote! {
            #ident::#variant => #display,
        }
    });

    if impl_default {
        item.attrs.push(parse_quote!(#[derive(Default)]));
    }

    item.attrs
        .push(parse_quote!(#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]));

    quote! {
        #item

        impl #ident {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #( #match_arms )*
                }
            }
        }

        impl AsRef<str> for #ident {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl crate::CssClass for #ident {
            fn class(&self) -> Option<&'static str> {
                Some(self.as_str())
            }
        }
    }
}
