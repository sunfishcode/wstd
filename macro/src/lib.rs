use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, ItemFn};

#[proc_macro_attribute]
pub fn attr_macro_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    if input.sig.asyncness.is_none() {
        return quote_spanned! { input.sig.fn_token.span()=>
            compile_error!("fn must be `async fn`");
        }
        .into();
    }

    if input.sig.ident != "main" {
        return quote_spanned! { input.sig.ident.span()=>
            compile_error!("only `async fn main` can be used for #[wstd::main]");
        }
        .into();
    }

    if !input.sig.inputs.is_empty() {
        return quote_spanned! { input.sig.inputs.span()=>
            compile_error!("arguments to main are not supported");
        }
        .into();
    }
    let attrs = input.attrs;
    let output = input.sig.output;
    let block = input.block;
    quote! {
        pub fn main() #output {

            #(#attrs)*
            async fn __run() #output {
                #block
            }

            ::wstd::runtime::block_on(async {
                __run().await
            })
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn attr_macro_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    if input.sig.asyncness.is_none() {
        return quote_spanned! { input.sig.fn_token.span()=>
            compile_error!("fn must be `async fn`");
        }
        .into();
    }

    let name = input.sig.ident;

    if !input.sig.inputs.is_empty() {
        return quote_spanned! { input.sig.inputs.span()=>
            compile_error!("arguments to main are not supported");
        }
        .into();
    }
    let attrs = input.attrs;
    let output = input.sig.output;
    let block = input.block;
    quote! {
        #[test]
        pub fn #name() #output {

            #(#attrs)*
            async fn __run() #output {
                #block
            }

            ::wstd::runtime::block_on(async {
                __run().await
            })
        }
    }
    .into()
}

/// Enables a HTTP-server main function, for creating [HTTP servers].
///
/// [HTTP servers]: https://docs.rs/wstd/latest/wstd/http/server/index.html
///
/// # Examples
///
/// ```ignore
/// #[wstd::http_server]
/// async fn main(request: Request<IncomingBody>, responder: Responder) -> Finished {
///     responder
///         .respond(Response::new("Hello!\n".into_body()))
///         .await
/// }
/// ```
#[proc_macro_attribute]
pub fn attr_macro_http_server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    if input.sig.asyncness.is_none() {
        return quote_spanned! { input.sig.fn_token.span()=>
            compile_error!("fn must be `async fn`");
        }
        .into();
    }

    let output = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if name != "main" {
        return quote_spanned! { input.sig.ident.span()=>
            compile_error!("only `async fn main` can be used for #[wstd::http_server]");
        }
        .into();
    }

    quote! {
        struct TheServer;

        impl ::wstd::wasi::exports::http::incoming_handler::Guest for TheServer {
            fn handle(
                request: ::wstd::wasi::http::types::IncomingRequest,
                response_out: ::wstd::wasi::http::types::ResponseOutparam
            ) {
                #(#attrs)*
                #vis async fn __run(#inputs) #output {
                    #body
                }

                let responder = ::wstd::http::server::Responder::new(response_out);
                let _finished: ::wstd::http::server::Finished =
                    match ::wstd::http::try_from_incoming_request(request)
                {
                    Ok(request) => ::wstd::runtime::block_on(async { __run(request, responder).await }),
                    Err(err) => responder.fail(err),
                };
            }
        }

        ::wstd::wasi::http::proxy::export!(TheServer with_types_in ::wstd::wasi);

        // In case the user needs it, provide a `main` function so that the
        // code compiles.
        fn main() { unreachable!("HTTP-server components should be run wth `handle` rather than `main`") }
    }
    .into()
}
