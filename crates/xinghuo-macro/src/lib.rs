mod element;
mod ui_view;

use ui_view::parse_ui_view;

use proc_macro::TokenStream;

#[proc_macro]
pub fn ui_view(tokens: TokenStream) -> TokenStream {
    match parse_ui_view(tokens) {
        Ok(out) => out,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

// #[proc_macro]
// pub fn ui_element(tokens: TokenStream) -> TokenStream {
//     match parse_ui_element(tokens) {
//         Ok(out) => out,
//         Err(e) => e.to_compile_error(),
//     }
//     .into()
// }
