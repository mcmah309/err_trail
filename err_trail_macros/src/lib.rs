extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;

mod input;
use input::{Format, Input};

fn generate_logger_impl(level: &str, args: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(args as Input);
    expand(level, input).into()
}

fn expand(level: &str, input: Input) -> TokenStream2 {
    if !cfg!(any(feature = "tracing", feature = "log", feature = "defmt")) {
        let fields = input.fields.iter().map(|field| {
            let value = &field.value;
            match field.format {
                Format::Value => quote!(let _ = &(#value);),
                // The extra reference supports unsized values such as str,
                // slices, and trait objects when coercing to the trait object.
                Format::Debug => quote!(let _: &dyn ::core::fmt::Debug = &&(#value);),
                Format::Display => quote!(let _: &dyn ::core::fmt::Display = &&(#value);),
            }
        });
        let target = input
            .target
            .as_ref()
            .map(|target| quote!(let _ = &(#target);));
        let message = input
            .message
            .as_ref()
            .map(|message| quote!(let _ = ::core::format_args!(#message);));
        // Count logging arguments as used without evaluating them. Borrowing
        // fields avoids moves; format_args! also handles implicit message
        // captures such as "{error}". Bare fields have backend-specific bounds.
        return quote! {{
            if false {
                #target
                #(#fields)*
                #message
            }
        }};
    }

    let backend = |name: &str| {
        let name = Ident::new(name, Span::call_site());
        match &input.root {
            Some(root) => quote!(#root::__private::#name),
            // Preserve direct usage of the err_trail_macros crate as well.
            None => quote!(::#name),
        }
    };
    let level_ident = Ident::new(level, Span::call_site());
    let message_ident = Ident::new("__err_trail_message", Span::mixed_site());
    let target = input
        .target
        .as_ref()
        .map_or_else(|| quote!(::core::module_path!()), |target| quote!(#target));
    // Inline consts enforce tracing's static target requirement without a
    // generated item that could shadow a caller's constant of the same name.
    let target = quote!({ const { ::core::convert::identity::<&str>(#target) } });

    let mut values = Vec::new();
    let mut bindings = Vec::new();
    let mut tracing_fields = Vec::new();
    let mut text_args = Vec::new();
    let mut text_format = String::new();
    if input.message.is_some() {
        text_format.push_str("{}");
        text_args.push(quote!(#message_ident));
        tracing_fields.push(quote!(message = #message_ident));
    }
    for (index, field) in input.fields.iter().enumerate() {
        let binding = Ident::new(&format!("__err_trail_field_{index}"), Span::mixed_site());
        let name = &field.name;
        let value = &field.value;
        let modifier = match field.format {
            Format::Value => quote!(),
            Format::Debug => quote!(?),
            Format::Display => quote!(%),
        };
        tracing_fields.push(quote!(#name = #modifier #binding));
        values.push(quote!(&(#value)));
        bindings.push(quote!(#binding));
        if !text_format.is_empty() {
            text_format.push(' ');
        }
        // Match tracing-subscriber's default text labels without changing the
        // structured field names supplied to tracing. Braces remain literal.
        let name = name.value();
        let name = name.strip_prefix("r#").unwrap_or(&name);
        text_format.push_str(&name.replace('{', "{{").replace('}', "}}"));
        text_format.push_str(match field.format {
            Format::Display => "={}",
            Format::Value | Format::Debug => "={:?}",
        });
        text_args.push(quote!(#binding));
    }
    if let Some(message) = input.message {
        values.push(quote!(::core::format_args!(#message)));
        bindings.push(quote!(#message_ident));
    }

    let mut statements = Vec::new();
    if cfg!(feature = "tracing") {
        let tracing = backend("tracing");
        let level = Ident::new(&level.to_uppercase(), Span::call_site());
        statements.push(quote! {
            #tracing::event!(target: #target, #tracing::Level::#level, {
                #(#tracing_fields,)*
            });
        });
    }
    if cfg!(feature = "log") {
        let log = backend("log");
        statements.push(quote! {
            #log::#level_ident!(target: #target, #text_format, #(#text_args),*);
        });
    }
    if cfg!(feature = "defmt") {
        let defmt = backend("defmt");
        let text = quote!(defmt::Display2Format(
            &::core::format_args!(#text_format, #(#text_args),*)
        ));
        // defmt has no target metadata. Preserve explicit targets using
        // tracing's default text style; otherwise leave the message unprefixed.
        let statement = if input.target.is_some() {
            quote!(defmt::#level_ident!("{}: {}", #target, #text);)
        } else {
            quote!(defmt::#level_ident!("{}", #text);)
        };
        // defmt's procedural macros themselves emit paths rooted at `defmt`.
        // Supply that name locally, without requiring a downstream dependency
        // or bringing it into scope while evaluating the user's expressions.
        statements.push(quote!({
            use #defmt as defmt;
            #statement
        }));
    }

    quote! {{
        // The match keeps temporary field values and format_args! operands
        // alive for every backend, without moving caller-owned values.
        match (#(#values,)*) {
            (#(#bindings,)*) => { #(#statements)* }
        }
    }}
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
