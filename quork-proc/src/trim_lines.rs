use proc_macro2::TokenStream;
use syn::LitStr;

pub enum Alignment {
    Left,
    Right,
    None,
}

pub fn trim_lines(literal: &LitStr, alignment: &Alignment) -> TokenStream {
    let value = literal.value();

    let mut lines = value.lines().collect::<Vec<&str>>();

    while let Some(first) = lines.first() {
        if first.is_empty() {
            lines.remove(0);
        } else {
            break;
        }
    }

    while let Some(last) = lines.last() {
        if last.is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    let stripped = lines
        .iter()
        .map(|line| match alignment {
            Alignment::Left => line.trim_start(),
            Alignment::Right => line.trim_end(),
            Alignment::None => line.trim(),
        })
        .collect::<Vec<_>>()
        .join("\n");

    let stripped_tokens = LitStr::new(&stripped, literal.span());

    quote! { #stripped_tokens }
}
