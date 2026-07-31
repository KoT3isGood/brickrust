use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn sig(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let pattern = input.value();

    let mut bytes = Vec::new();
    let mut mask = Vec::new();

    for part in pattern.split_whitespace() {
        if part == "??" || part == "?" {
            bytes.push(0u8);
            mask.push(false);
        } else {
            let byte = u8::from_str_radix(part, 16)
                .expect("invalid byte in signature");

            bytes.push(byte);
            mask.push(true);
        }
    }

    let byte_tokens = bytes.iter();
    let mask_tokens = mask.iter();

    quote! {
        Signature {
            bytes: &[#(#byte_tokens),*],
            mask: &[#(#mask_tokens),*],
        }
    }
    .into()
}
