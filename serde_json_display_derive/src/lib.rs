extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsonDisplay)]
pub fn json_display_derive(input: TokenStream) -> TokenStream {
    // 解析输入
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // 生成代码
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let json_str = serde_json::to_string(self).unwrap_or_else(|_| "format json failed".to_string());
                write!(f, "{}", json_str)
            }
        }
    };

    TokenStream::from(expanded)
}