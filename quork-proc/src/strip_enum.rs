use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, DeriveInput, Expr, ExprLit, Lit, Meta, Token,
    Variant, Visibility,
};

fn ignore_variant(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| match attr.meta {
        syn::Meta::Path(ref p) => p.is_ident("stripped_ignore"),
        _ => abort!(
            attr.span(),
            "Expected path-style (i.e #[stripped_ignore]), found other style attribute macro"
        ),
    })
}

struct StrippedData {
    ident: Ident,
    variants: Vec<TokenStream>,
    meta: Vec<Expr>,
    vis: Visibility,
}

// struct MetaArgs {
//     meta: Vec<Expr>,
// }

// impl Parse for MetaArgs {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//          input.peek3(token)
//     }
// }

pub fn strip_enum(ast: &DeriveInput) -> TokenStream {
    let data = &ast.data;
    let attrs = &ast.attrs;

    let info: StrippedData = match data {
        syn::Data::Enum(ref e) => {
            let variants = e
                .variants
                .iter()
                .filter_map(|variant| {
                    if ignore_variant(variant) {
                        None
                    } else {
                        Some(variant.ident.to_token_stream())
                    }
                })
                .collect::<Vec<_>>();

            let default_ident = || Ident::new(&format!("{}Stripped", ast.ident), ast.ident.span());

            let (new_ident, ignored) = if let Some(info_attr) =
                attrs.iter().find(|attr| attr.path().is_ident("stripped"))
            {
                let mut new_ident: Option<Ident> = None;
                let mut ignored = false;

                syn::meta::parser(|meta| {
                    if meta.path.is_ident("ident") {
                        new_ident = Some(meta.value()?.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("ignore") {
                        ignored = true;
                        Ok(())
                    } else {
                        Err(meta.error("unsupported stripped property"))
                    }
                });

                (new_ident.unwrap_or_else(default_ident), ignored)
            } else {
                (default_ident(), false)
            };

            // let meta = attrs
            //     .iter()
            //     .filter(|attr| attr.path().is_ident("stripped_meta"))
            //     .map(|attr| match &attr.meta {
            //         // TODO: Add inherit metadata
            //         syn::Meta::List(meta) => meta.parse_args().expect(&*"single meta attribute"),
            //         _ => abort!(
            //             attr.span(),
            //             "Expected #[stripped_meta(...)]. Found other style attribute."
            //         ),
            //     })
            //     .collect();

            StrippedData {
                ident: new_ident,
                variants,
                meta: ast.,
                vis: ast.vis.clone(),
            }
        }
        _ => abort_call_site!("`Strip` can only be derived for enums"),
    };

    let StrippedData {
        ident,
        variants,
        meta,
        vis,
    } = info;

    quote! {
        #(#[#meta])*
        #vis enum #ident {
            #(#variants),*
        }
    }
}
