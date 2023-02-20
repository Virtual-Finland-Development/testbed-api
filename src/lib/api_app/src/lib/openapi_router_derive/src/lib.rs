//#![feature(trace_macros)]
//trace_macros!(true);

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

    // Map the operation_id to the operation function
    let operations = operation_function_paths
        .iter()
        .map(|path| {
            let operation_id = path
                .segments
                .last()
                .expect("Expected at least one segment")
                .ident
                .to_string();
            let operation = path.to_token_stream();
            // output eg: "index" => application::index(parsed_request).await,
            quote! {
                #operation_id => #operation(parsed_request).await,
            }
        })
        .collect::<Vec<_>>();

    // Create the impl block
    let expanded = quote! {
        impl OpenApiRouter for #ident {
            type FutureType = BoxFuture<'static, APIResponse>;
            fn get_operation(&self, operation_id: String, parsed_request: ParsedRequest) -> Box<dyn FnOnce() -> Self::FutureType + Send> {
                Box::new(move || async move {
                        match operation_id.as_str() {
                            #(#operations)*
                            _ => application::not_found(parsed_request).await, // Catch all 404
                        }
                    }.boxed()
                )
           }
        }
    };

    TokenStream::from(expanded)
}
