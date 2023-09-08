use proc_macro2::TokenStream;

pub fn token_stream_with(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(error.into_compile_error());
    tokens
}

macro_rules! macro_error {
    ($($tt:tt)*) => {
        quote! {compile_error!(stringify!($($tt)*))}
    };
}
