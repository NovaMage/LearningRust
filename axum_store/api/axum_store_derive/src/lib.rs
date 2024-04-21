extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2;
use syn::DeriveInput;

#[proc_macro_derive(Output, attributes(StatusCode))]
pub fn output_deriver(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = expand_output_derive(&ast);

    // Return the generated impl
    gen.into()
}

fn expand_output_derive(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                axum::response::Json(self).into_response()
            }
        }
    }
}
