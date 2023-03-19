use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Ident};

#[proc_macro_derive(BindFields)]
pub fn bind_fields_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let span = ast.span();

    let name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let cratee = lanime_bindfields_path();

    let inner = match ast.data {
        syn::Data::Struct(d) => {
            let mut counter = 0;
            let mut res = quote! {};
            for field in d.fields {
                let vis = field.vis;
                let ty = field.ty;
                if let Some(name) = field.ident {
                    //pub const x: BindableField<Self, f32> = BindableField(|me| &mut me.x);
                    res = quote! {
                        #res
                        #vis const #name: #cratee::BindableField<Self, #ty> = #cratee::BindableField(|me| &mut me.#name);
                    };
                } else {
                    let name = format!("field_{counter}");
                    res = quote! {
                        #res
                        #vis const #name: #cratee::BindableField<Self, #ty> = #cratee::BindableField(|me| &mut me.#counter);
                    };
                }
                counter += 1;
            }
            res
        }
        _ => quote_spanned! {
            span => compile_error!("BindFields can only be applied to structs.")
        },
    };

    quote! {
        #[allow(non_upper_case_globals)]
        impl #impl_generics #name #ty_generics #where_clause {
            #inner
        }
    }
    .into()
}

pub(crate) fn lanime_bindfields_path() -> impl ToTokens {
    let found_crate =
        crate_name("lanime_bindfields").expect("lanime_bindfields is not present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => quote!(::crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(::#ident)
        }
    }
}
