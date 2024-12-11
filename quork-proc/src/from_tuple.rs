use proc_macro2::Span;
use proc_macro_error2::abort_call_site;
use syn::{DeriveInput, Ident, Type};

pub fn derive(input: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let data = &input.data;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut fields = Vec::new();

    match data {
        syn::Data::Struct(data) => {
            for field in &data.fields {
                fields.push((field.ident.clone(), field.ty.clone()));
            }
        }
        _ => abort_call_site!("Should only be derived from a struct"),
    }

    let (mut idents, types): (Vec<Option<Ident>>, Vec<Type>) = fields.into_iter().unzip();
    let is_tuple_struct = idents.contains(&None);

    for (index, ident) in &mut idents.iter_mut().enumerate() {
        if ident.is_none() {
            *ident = Some(Ident::new(&format!("field{index}"), Span::call_site()));
        }
    }

    // Check if the struct contains fields without identifiers (usually meaning a tuple struct)
    let struct_instantiation = if is_tuple_struct {
        quote! {
            Self( #(#idents),* )
        }
    } else {
        quote! {
                Self { #(#idents),* }
        }
    };

    quote! {
        impl #impl_generics From<(#(#types),*)> for #ident #ty_generics #where_clause {
            fn from((#(#idents),*): (#(#types),*)) -> Self {
                #struct_instantiation
            }
        }
    }
}
