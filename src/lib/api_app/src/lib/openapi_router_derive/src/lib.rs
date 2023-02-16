// lib.rs
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Meta, MetaList, NestedMeta};

#[proc_macro_derive(OpenApiRouter)]
pub fn derive_openapi_router(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, ident, .. } = parse_macro_input!(input);

    // Collect functions from the #[openapi(paths(...))] attribute
    let openapi_attr = attrs
        .iter()
        .find(|a| a.path.is_ident("openapi"))
        .expect("Expected #[openapi(...)] attribute");

    // Resolve actual function paths
    let mut operation_function_paths = Vec::new();
    if let Ok(Meta::List(MetaList { nested, .. })) = openapi_attr.parse_meta() {
        for item in nested {
            if let NestedMeta::Meta(Meta::List(MetaList { path, nested, .. })) = item {
                if let Some(ident) = path.get_ident() {
                    if ident == "paths" {
                        for path in nested {
                            if let NestedMeta::Meta(Meta::Path(path)) = path {
                                operation_function_paths.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    // Generate a comma separated list of function paths
    let operation_function_paths_str = operation_function_paths
        .iter()
        .map(|path| path.to_token_stream().to_string())
        .collect::<Vec<String>>()
        .join(", ");

    // @TODO: gerenerate function calls for each path
    let expanded = quote! {
        impl OpenApiRouter for #ident {
            fn handle_operation(operation_id: String) {
                println!("all operation paths: {}", #operation_function_paths_str);
                println!("hello from operation: {}", operation_id);
           }
        }
    };

    TokenStream::from(expanded)
}
