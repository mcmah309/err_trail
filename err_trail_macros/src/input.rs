use proc_macro2::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, LitStr, Result, Token, braced, bracketed, token};

pub struct Input {
    // Forwarded by err_trail's macro_rules wrappers. Keep $crate opaque so its
    // hygiene survives the round trip through the procedural macro.
    pub root: Option<TokenStream>,
    pub target: Option<Expr>,
    pub fields: Vec<Field>,
    pub message: Option<TokenStream>,
}

pub struct Field {
    pub name: LitStr,
    pub value: Expr,
    pub format: Format,
}

#[derive(Clone, Copy)]
pub enum Format {
    Value,
    Debug,
    Display,
}

impl Format {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Ok(Self::Debug)
        } else if input.peek(Token![%]) {
            input.parse::<Token![%]>()?;
            Ok(Self::Display)
        } else {
            Ok(Self::Value)
        }
    }
}

// A single colon introduces metadata; a double colon may start a message
// macro path such as core::concat!(...).
fn metadata(input: ParseStream) -> bool {
    input.peek(Ident::peek_any) && input.peek2(Token![:]) && !input.peek2(Token![::])
}

fn reject_metadata(input: ParseStream) -> Result<()> {
    if metadata(input) {
        let key = input.call(Ident::parse_any)?;
        return Err(syn::Error::new(
            key.span(),
            if key == "target" {
                "target: must appear once, before the fields and message"
            } else {
                "unsupported logging metadata; only target: is supported"
            },
        ));
    }
    Ok(())
}

fn field_start(input: ParseStream) -> Result<bool> {
    if input.peek(Token![?]) || input.peek(Token![%]) {
        return Ok(true);
    }
    if input.peek(LitStr) {
        return Ok(input.peek2(Token![=]));
    }
    if input.peek(Ident::peek_any) {
        let lookahead = input.fork();
        lookahead.call(Ident::parse_any)?;
        while lookahead.peek(Token![.]) {
            lookahead.parse::<Token![.]>()?;
            lookahead.call(Ident::parse_any)?;
        }
        return Ok(lookahead.is_empty() || lookahead.peek(Token![,]) || lookahead.peek(Token![=]));
    }
    Ok(false)
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        reject_metadata(input)?;
        if input.peek(token::Brace) {
            return Err(input.error("constant-expression field names are not supported"));
        }

        let shorthand_format = Format::parse(input)?;
        let (name, shorthand) = if input.peek(LitStr) {
            (input.parse::<LitStr>()?, None)
        } else {
            let first = input.call(Ident::parse_any)?;
            let mut name = first.to_string();
            let mut value = quote!(#first);
            while input.peek(Token![.]) {
                input.parse::<Token![.]>()?;
                let member = input.call(Ident::parse_any)?;
                name.push('.');
                name.push_str(&member.to_string());
                value.extend(quote!(.#member));
            }
            (LitStr::new(&name, first.span()), Some(value))
        };

        let (format, value) = if input.peek(Token![=]) {
            if !matches!(shorthand_format, Format::Value) {
                return Err(input
                    .error("put the field modifier after '=': field = ?value or field = %value"));
            }
            input.parse::<Token![=]>()?;
            (Format::parse(input)?, input.parse()?)
        } else if let Some(value) = shorthand {
            if !input.is_empty() && !input.peek(Token![,]) {
                return Err(input.error("expected ',' after field; use field = %expression or field = ?expression for computed values"));
            }
            (shorthand_format, syn::parse2(value)?)
        } else {
            return Err(input.error("expected '=' after a quoted field name"));
        };

        Ok(Self {
            name,
            value,
            format,
        })
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let root = if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            let root;
            bracketed!(root in input);
            Some(root.parse()?)
        } else {
            None
        };

        let target = if metadata(input) {
            let key = input.call(Ident::parse_any)?;
            if key != "target" {
                return Err(syn::Error::new(
                    key.span(),
                    "unsupported logging metadata; only target: is supported",
                ));
            }
            input.parse::<Token![:]>()?;
            let target = input.parse()?;
            input.parse::<Token![,]>()?;
            Some(target)
        } else {
            None
        };

        let mut fields = Vec::new();
        let message;
        if input.peek(token::Brace) {
            let group;
            braced!(group in input);
            if input.peek(Token![=]) {
                return Err(input.error("constant-expression field names are not supported"));
            }
            while !group.is_empty() {
                fields.push(group.parse()?);
                if !group.is_empty() {
                    group.parse::<Token![,]>()?;
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
            reject_metadata(input)?;
            message = if input.is_empty() {
                None
            } else {
                Some(input.parse()?)
            };
        } else {
            loop {
                reject_metadata(input)?;
                if input.peek(token::Brace) {
                    return Err(input.error("constant-expression field names are not supported"));
                }
                if input.is_empty() || !field_start(input)? {
                    break;
                }
                fields.push(input.parse()?);
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
            }
            message = if input.is_empty() {
                None
            } else {
                Some(input.parse()?)
            };
        }

        if fields.is_empty() && message.is_none() {
            return Err(input.error("expected at least one field or a format string"));
        }

        Ok(Self {
            root,
            target,
            fields,
            message,
        })
    }
}
