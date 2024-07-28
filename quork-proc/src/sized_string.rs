use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use syn::Ident;

pub fn sized_string(input: &syn::LitStr) -> proc_macro2::TokenStream {
    let length = input.value().chars().count();
    let chars = input
        .value()
        .chars()
        .map(|c| quote::quote! { #c as u8 })
        .collect::<Vec<_>>();

    let quork_crate =
        match proc_macro_crate::crate_name("quork").expect("quork is present in `Cargo.toml`") {
            FoundCrate::Itself => quote::quote! { crate },
            FoundCrate::Name(name) => {
                let ident = Ident::new(&name, Span::call_site());
                quote::quote! { #ident }
            }
        };

    let output = quote::quote! {
        #quork_crate::sized_string::SizedString::<#length>::new([#(#chars),*])
    };

    output
}
