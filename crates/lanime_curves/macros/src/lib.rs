use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    LitInt, Result, Token,
};

struct RecursiveTool {
    start_idx: usize,
    len: usize,
}

impl Parse for RecursiveTool {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit: LitInt = input.parse()?;
        let start_idx = lit.base10_parse::<usize>()?;
        input.parse::<Token![,]>()?;
        let lit: LitInt = input.parse()?;
        let len = lit.base10_parse::<usize>()?;
        Ok(Self { start_idx, len })
    }
}

#[proc_macro]
pub fn recursive_tool(input: TokenStream) -> TokenStream {
    let data: RecursiveTool = syn::parse(input).unwrap();

    let start_idx = data.start_idx;
    let len = data.len;

    if len == 1 {
        quote! {
            self[#start_idx]
        }
    } else {
        let new_len = len - 1;
        let other_start_idx = start_idx + 1;
        quote! {
            (1. - t) * (::lanime_curves_macros::recursive_tool!(#start_idx, #new_len)) + t * (::lanime_curves_macros::recursive_tool!(#other_start_idx, #new_len))
        }
    }
    .into()
}
