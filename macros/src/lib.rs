use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr };

fn signature(input: LitStr) -> (Vec<u8>, Vec<bool>)
{
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
    (bytes, mask)
}

#[proc_macro]
pub fn sig(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let (bytes, mask) = signature(input);
    let bytes = bytes.iter();
    let mask = mask.iter();

    quote! {
        Signature {
            bytes: &[#(#bytes),*],
            mask: &[#(#mask),*],
        }
    }
    .into()
}
