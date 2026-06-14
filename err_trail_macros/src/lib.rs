extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

fn generate_logger_impl(level: &str, args: TokenStream) -> TokenStream {
    let args_tokens = proc_macro2::TokenStream::from(args);
    let level_ident = syn::Ident::new(level, proc_macro2::Span::call_site());

    let mut statements: Vec<proc_macro2::TokenStream> = Vec::new();

    #[cfg(feature = "tracing")]
    statements.push(quote! {
        tracing::#level_ident!("{}", args);
    });
    #[cfg(feature = "log")]
    statements.push(quote! {
        log::#level_ident!("{}", args);
    });
    #[cfg(feature = "defmt")]
    statements.push(quote! {
        defmt::#level_ident!("{}", defmt::Display2Format(&args));
    });

    #[cfg(any(feature = "tracing", feature = "log", feature = "defmt"))]
    let expanded = quote! {{
        #[allow(unused_variables)]
        let args = format_args!(#args_tokens);
        #(#statements)*
    }};
    #[cfg(not(any(feature = "tracing", feature = "log", feature = "defmt")))]
    let expanded = quote! {};

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    generate_logger_impl("error", input)
}

#[proc_macro]
pub fn warn(input: TokenStream) -> TokenStream {
    generate_logger_impl("warn", input)
}

#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    generate_logger_impl("info", input)
}

#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    generate_logger_impl("debug", input)
}

#[proc_macro]
pub fn trace(input: TokenStream) -> TokenStream {
    generate_logger_impl("trace", input)
}
