use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

macro_rules! define_route_macro {
    ($macro_name:ident, $method_str:expr) => {
        #[proc_macro_attribute]
        pub fn $macro_name(args: TokenStream, input: TokenStream) -> TokenStream {
            let path = match syn::parse::<LitStr>(args) {
                Ok(lit) => lit.value(),
                Err(_) => {
                    let err = syn::Error::new(
                        proc_macro2::Span::call_site(),
                        concat!("Expected string literal for path, e.g. #[", stringify!($macro_name), "(\"/path\")]")
                    );
                    return err.to_compile_error().into();
                }
            };

            let input_fn = parse_macro_input!(input as ItemFn);
            let fn_name = &input_fn.sig.ident;
            
            let wrapper_name = format_ident!("__route_wrapper_{}_{}", $method_str.to_lowercase(), fn_name);

            let expanded = quote! {
                #input_fn

                #[doc(hidden)]
                pub fn #wrapper_name() -> really_fast_api::router::RouteHandler {
                    std::sync::Arc::new(move |req| Box::pin(#fn_name(req)))
                }

                inventory::submit! {
                    really_fast_api::router::RouteRegistration {
                        method: $method_str,
                        path: #path,
                        handler_fn: #wrapper_name,
                    }
                }
            };

            TokenStream::from(expanded)
        }
    };
}

define_route_macro!(get, "GET");
define_route_macro!(post, "POST");
define_route_macro!(put, "PUT");
define_route_macro!(delete, "DELETE");