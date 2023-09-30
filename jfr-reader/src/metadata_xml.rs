// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Interact with metadata.xml files.

use convert_case::{Case, Casing};
use jfr_metadata_xml::schema::{Event, Field, Metadata, Type};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::parse_quote;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("metadata error: {0}")]
    Metadata(#[from] jfr_metadata_xml::Error),

    #[error("field type not found: {0}")]
    FieldTypeNotFound(String),

    #[error("type not found: {0}")]
    TypeNotFound(String),

    #[error("syntax error: {0}")]
    Syn(#[from] syn::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// Derive the name of a Rust type for a given type name.
pub fn type_to_rust_type<'a>(m: &'a Metadata, name: &str) -> Option<(&'a str, bool)> {
    // Type are mapped to structs having that name.
    if let Some(t) = m.find_type(name) {
        Some((t.name.as_str(), true))
    } else if let Some(xml_type) = m.find_xml_type(name) {
        if let Some(java_type) = xml_type.java_type.as_ref().map(|x| x.as_str()) {
            match (java_type, xml_type.unsigned.unwrap_or_default()) {
                ("byte", false) => Some(("i8", false)),
                ("byte", true) => Some(("u8", false)),
                ("short", false) => Some(("i16", false)),
                ("short", true) => Some(("u16", false)),
                ("int", false) => Some(("i32", false)),
                ("int", true) => Some(("u32", false)),
                ("long", false) => Some(("i64", false)),
                ("long", true) => Some(("u64", false)),
                ("float", _) => Some(("f32", false)),
                ("double", _) => Some(("f64", false)),
                ("boolean", _) => Some(("bool", false)),
                ("char", _) => Some(("char", false)),
                ("java.lang.String", _) => Some(("String", true)),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Derive the value type for a struct field.
pub fn field_type(m: &Metadata, type_name: &str, field: &Field) -> Result<TokenStream> {
    let (rust_type, is_nullable) = type_to_rust_type(m, &field.typ)
        .ok_or_else(|| Error::FieldTypeNotFound(field.typ.clone()))?;
    let rust_type = format_ident!("{}", rust_type);

    Ok(if field.array.unwrap_or_default() {
        quote! { Vec<#rust_type> }
    } else {
        // Some types have recursive references to other types. Break those.
        // TODO the hardcoded exceptions here are a bit hacky.
        let rust_type = if (type_name == "ThreadGroup" && field.name == "parent")
            || (type_name == "ClassLoader" && field.name == "type")
            || (type_name == "OldObject" && field.name == "referrer")
        {
            quote! { Box<#rust_type> }
        } else {
            quote! { #rust_type }
        };

        if is_nullable {
            quote! { Option<#rust_type> }
        } else {
            rust_type
        }
    })
}

/// Resolve an identifier for a struct field.
pub fn field_identifier(name: &str) -> Ident {
    let name = name.to_case(Case::Snake);

    // TODO there's got to be a better way.
    if matches!(name.as_str(), "type" | "virtual") {
        format_ident!("r#{}", name)
    } else {
        format_ident!("{}", name)
    }
}

/// Resolve tokens for a struct's field.
pub fn struct_field(m: &Metadata, type_name: &str, field: &Field) -> Result<TokenStream> {
    let name_original = field.name.as_str();
    let name = field_identifier(name_original);

    let doc = if let Some(desc) = &field.description {
        desc
    } else {
        &field.label
    };

    let rust_type = field_type(m, type_name, field)?;

    Ok(quote! {
        #[doc = #doc]
        #[serde(rename = #name_original)]
        pub #name: #rust_type
    })
}

/// Resolve tokens for a [Type] struct.
pub fn type_struct(m: &Metadata, typ: &Type) -> Result<TokenStream> {
    let name = format_ident!("{}", typ.name);

    let doc = if let Some(label) = &typ.label {
        quote! { #[doc = #label] }
    } else {
        quote! {}
    };

    let fields = typ
        .fields
        .iter()
        .map(|field| struct_field(m, &typ.name, field))
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #doc
        #[derive(Clone, Debug, Deserialize)]
        pub struct #name {
            #(#fields),*
        }
    })
}

/// Resolve tokens for an enum defining non-event types.
pub fn types_enum(m: &Metadata) -> Result<TokenStream> {
    let variants = m
        .types_sorted()
        .into_iter()
        .map(|t| {
            let name = format_ident!("{}", t.name);

            quote! { #name(#name) }
        })
        .collect::<Vec<_>>();

    let doc = "All non-event types";

    Ok(quote! {
        #[doc = #doc]
        #[derive(Clone, Debug, Deserialize)]
        pub enum Types {
            #(#variants),*
        }
    })
}

/// Resolve tokens for an [Event] struct.
pub fn event_struct(
    m: &Metadata,
    event: &Event,
    thread_type: &str,
    stack_trace_type: &str,
) -> Result<TokenStream> {
    let name = format_ident!("{}", event.name);
    let description = event
        .description
        .clone()
        .unwrap_or_else(|| event.name.clone());
    let thread_type_ident = format_ident!("{}", thread_type);
    let stack_trace_type_ident = format_ident!("{}", stack_trace_type);

    // Common fields are handled by the CommonFields struct, which is
    // descended into during deserialization.
    let mut fields = vec![quote! {
        #[serde(flatten)]
        common: crate::event::CommonFields<#thread_type_ident, #stack_trace_type_ident>
    }];

    fields.extend(
        event
            .fields
            .iter()
            .map(|field| struct_field(m, &event.name, field))
            .collect::<Result<Vec<_>>>()?
            .into_iter(),
    );

    Ok(quote! {
        #[doc = #description]
        #[derive(Clone, Debug, Deserialize)]
        pub struct #name {
            #(#fields),*
        }
    })
}

/// Resolve tokens for the Event implemnentation for an event struct.
pub fn event_event_impl(
    event: &Event,
    thread_type: &str,
    stack_trace_type: &str,
) -> Result<TokenStream> {
    let raw_name = event.name.as_str();
    let name = format_ident!("{}", event.name);
    let thread_type_ident = format_ident!("{}", thread_type);
    let stack_trace_type_ident = format_ident!("{}", stack_trace_type);

    Ok(quote! {
        impl<'slf> EventType<'slf, #thread_type_ident, #stack_trace_type_ident> for #name {
            const NAME: &'static str = #raw_name;

            fn common_fields(&'slf self) -> &'slf crate::event::CommonFields<#thread_type_ident, #stack_trace_type_ident> {
                &self.common
            }
        }
    })
}

/// Resolve tokens for an enum defining all events.
pub fn events_enum(m: &Metadata) -> Result<TokenStream> {
    let variants = m
        .events_sorted()
        .into_iter()
        .map(|e| {
            let name = format_ident!("{}", e.name);

            quote! { #name(#name) }
        })
        .collect::<Vec<_>>();

    let doc = "All events";

    Ok(quote! {
        #[doc = #doc]
        #[derive(Clone, Debug, Deserialize)]
        pub enum Events {
            #(#variants),*
        }
    })
}

/// Obtain .rs source code derived from a [Metadata] instance.
pub fn metadata_to_rs(m: &Metadata) -> Result<String> {
    let mut items = vec![
        parse_quote! { use crate::event::EventType; },
        parse_quote! { use serde::Deserialize; },
    ];

    // Make sure we thread and stack trace types.
    let thread_type = m
        .types()
        .find_map(|t| {
            if t.name == "Thread" {
                Some(t.name.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| Error::TypeNotFound("Thread".to_string()))?;

    let stack_trace_type = m
        .types()
        .find_map(|t| {
            if t.name == "StackTrace" {
                Some(t.name.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| Error::TypeNotFound("StackTrace".to_string()))?;

    // Emit non-event types/structs first.
    for typ in m.types_sorted() {
        items.push(syn::parse2(type_struct(m, typ)?)?);
    }

    // An enum for non-event types.
    items.push(syn::parse2(types_enum(m)?)?);

    // Now all the events.
    for e in m.events_sorted() {
        items.push(syn::parse2(event_struct(
            m,
            e,
            &thread_type,
            &stack_trace_type,
        )?)?);
        items.push(syn::parse2(event_event_impl(
            e,
            &thread_type,
            &stack_trace_type,
        )?)?);
    }

    // An enum for all events.
    items.push(syn::parse2(events_enum(m)?)?);

    let f = syn::File {
        shebang: None,
        attrs: vec![],
        items,
    };

    Ok(prettyplease::unparse(&f))
}
