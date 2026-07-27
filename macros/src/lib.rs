use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn uclass_game(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let n = &input.ident;
    
    let expanded = quote! {
        #input



    };

    TokenStream::from(expanded)
}
