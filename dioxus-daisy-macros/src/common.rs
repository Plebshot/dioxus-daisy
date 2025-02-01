use darling::ast::NestedMeta;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Token;

pub(crate) fn parse_meta_list(input: &ParseStream) -> syn::Result<Vec<NestedMeta>> {
    let list = Punctuated::<NestedMeta, Token![,]>::parse_terminated(input)?
        .into_iter()
        .collect();

    Ok(list)
}
