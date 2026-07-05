/// Attribute macro for defining commands
/// This creates a companion function that accepts a serde_json::Value Object as the parameters
/// and attempts to correctly deserialise and map them to the original function's arguments.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;
    let fn_name = sig.ident;

    // Collect args: (name, type)
    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    for input in sig.inputs.iter() {
        match input {
            syn::FnArg::Typed(pat_type) => {
                let name = match &*pat_type.pat {
                    syn::Pat::Ident(i) => i.ident.clone(),
                    _ => {
                        return syn::Error::new_spanned(
                            &pat_type.pat,
                            "unsupported argument pattern"
                        )
                            .to_compile_error()
                            .into();
                    }
                };

                arg_names.push(name);
                arg_types.push(&pat_type.ty);
            }
            _ => {
                return syn::Error::new_spanned(
                    input,
                    "unsupported receiver type"
                )
                    .to_compile_error()
                    .into();
            }
        }
    }

    let expanded = quote! {
        #vis async fn #fn_name(args: serde_json::Value)
            -> Result<serde_json::Value, String>
        {
            let obj = args
                .as_object()
                .ok_or("args must be JSON object")?;

            // Check for Unexpected Keys
            let mut expected_keys = vec![
                #(stringify!(#arg_names)),*
            ];

            for key in obj.keys() {
                if !expected_keys.contains(&key.as_str()) {
                    return Err(format!("unexpected argument: {}", key));
                }
            }

            // Check for Missing Keys
            for expected in &expected_keys {
                if !obj.contains_key(*expected) {
                    return Err(format!("missing argument: {}", expected));
                }
            }

            // Deserialise the Arguments
            #(let #arg_names: #arg_types =
                serde_json::from_value(
                    obj.get(stringify!(#arg_names))
                        .unwrap()
                        .clone()
                )
                .map_err(|e| e.to_string())?;
            )*

            let result = async move #block.await;
            serde_json::to_value(result)
                .map_err(|e| e.to_string())
        }
    };

    expanded.into()
}