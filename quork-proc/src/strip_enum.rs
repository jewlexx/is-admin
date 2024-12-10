use proc_macro2::{Ident, TokenStream};
use proc_macro_error2::{abort, abort_call_site};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, DeriveInput, Meta, MetaNameValue, Variant, Visibility};

fn ignore_variant(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| match attr.meta {
        syn::Meta::List(ref list) if list.path.is_ident("stripped") => {
            let mut ignored = false;

            let list_parser = syn::meta::parser(|meta| {
                if meta.path.is_ident("ignore") {
                    ignored = true;
                    Ok(())
                } else {
                    Err(meta.error("unsupported stripped property"))
                }
            });

            list.parse_args_with(list_parser).unwrap();

            ignored
        }
        _ => abort!(
            attr.span(),
            "Expected list-style (i.e #[stripped(...)]), found other style attribute macro"
        ),
    })
}

struct StrippedData {
    ident: Ident,
    variants: Vec<TokenStream>,
    meta: Vec<Meta>,
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

pub fn strip_enum(ast: &mut DeriveInput) -> TokenStream {
    let data = &ast.data;
    let attrs = &mut ast.attrs;

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

            let default_ident = {
                let ident = ast.ident.clone();
                let span = ident.span();
                move || Ident::new(&format!("{ident}Stripped"), span)
            };

            let new_ident = if let Some(info_attr_pos) = attrs
                .iter()
                .position(|attr| attr.path().is_ident("stripped"))
            {
                let info_attr = attrs.remove(info_attr_pos);

                let mut new_ident: Option<Ident> = None;

                let ident_parser = syn::meta::parser(|meta| {
                    if meta.path.is_ident("ident") {
                        new_ident = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error("unsupported stripped property"))
                    }
                });

                info_attr.parse_args_with(ident_parser).unwrap();

                new_ident.unwrap_or_else(default_ident)
            } else {
                default_ident()
            };

            let meta_list: Vec<syn::Meta> = attrs
                .iter()
                .filter(|attr| attr.path().is_ident("stripped_meta"))
                .flat_map(|meta_attr| match &meta_attr.meta {
                    Meta::List(meta_data) => vec![meta_data.parse_args::<syn::Meta>().unwrap()],
                    // Meta::NameValue(MetaNameValue {
                    //     value:
                    //         syn::Expr::Lit(syn::ExprLit {
                    //             lit: syn::Lit::Str(path),
                    //             ..
                    //         }),
                    //     ..
                    // }) => {
                    //     if &path.value() == "inherit" {
                    //         attrs
                    //             .iter()
                    //             .filter(|attr| !attr.path().is_ident("stripped_meta"))
                    //             .map(|attr| attr.meta.clone())
                    //             .collect()
                    //     } else {
                    //         abort!(path.span(), "Expected `inherit`");
                    //     }
                    // }
                    _ => abort!(
                        meta_attr.span(),
                        "Expected #[stripped_meta(...)]. Found other style attribute."
                    ),
                })
                .collect();

            StrippedData {
                ident: new_ident,
                variants,
                meta: meta_list,
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

    // panic!("{:?}", meta);

    quote! {
        #(#[#meta])*
        #vis enum #ident {
            #(#variants),*
        }
    }
}
