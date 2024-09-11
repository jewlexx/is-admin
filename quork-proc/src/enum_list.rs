use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error2::{abort_if_dirty, emit_error};
use syn::{spanned::Spanned, Data, DeriveInput};

use proc_macro_crate::{crate_name, FoundCrate};

pub fn enum_list(ast: &DeriveInput) -> TokenStream {
    let found_crate = crate_name("quork").expect("quork is present in `Cargo.toml`");

    let trait_ident = match found_crate {
        FoundCrate::Itself => quote!(crate::traits::list::ListVariants),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::traits::list::ListVariants )
        }
    };

    let ident = &ast.ident;

    let variants = match &ast.data {
        Data::Enum(enum_data) => {
            let variants = enum_data.variants.iter();

            variants
                .map(|var| {
                    let var_ident = &var.ident;
                    if !var.fields.is_empty() {
                        emit_error!(
                            var.span(),
                            "This variant has one or more fields. Variants cannot have any fields."
                        );
                    }

                    quote! ( #ident::#var_ident )
                })
                .collect::<Vec<_>>()
        }
        _ => proc_macro_error2::abort_call_site!("Can only be derived on an enum"),
    };

    abort_if_dirty();

    let variants_size = variants.len();

    quote::quote! {
        impl #trait_ident<#variants_size> for #ident {
            const VARIANTS: [Self;#variants_size] = [#(#variants,)*];
        }
    }
}
