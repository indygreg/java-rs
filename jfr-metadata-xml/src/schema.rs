// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR metadata XML schema.

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PeriodType {
    BeginChunk,
    EndChunk,
    EveryChunk,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransitionType {
    From,
    To,
}

/// Schema for a JFR metadata XML file.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(default, rename = "$value")]
    pub elements: Vec<RootElement>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum RootElement {
    Event(Event),
    Type(Type),
    XmlType(XmlType),
    XmlContentType(XmlContentType),
    Relation(Relation),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(rename = "Field", default)]
    pub fields: Vec<Field>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@category")]
    pub category: String,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@experimental")]
    pub experimental: Option<bool>,
    #[serde(rename = "@internal")]
    pub internal: Option<bool>,
    #[serde(rename = "@thread")]
    pub thread: Option<bool>,
    #[serde(rename = "@startTime")]
    pub start_time: Option<bool>,
    #[serde(rename = "@stackTrace")]
    pub stack_trace: Option<bool>,
    #[serde(rename = "@period")]
    pub period: Option<PeriodType>,
    #[serde(rename = "@cutoff")]
    pub cutoff: Option<bool>,
    #[serde(rename = "@throttle")]
    pub throttle: Option<bool>,
    /// Deprecated in JDK 19.
    #[serde(rename = "@commitState")]
    pub commit_state: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Type {
    #[serde(rename = "Field", default)]
    pub fields: Vec<Field>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@experimental")]
    pub experimental: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    #[serde(rename = "@type")]
    pub typ: String,
    #[serde(rename = "@struct")]
    pub strukt: Option<bool>,
    #[serde(rename = "@array")]
    pub array: Option<bool>,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@experimental")]
    pub experimental: Option<bool>,
    #[serde(rename = "@internal")]
    pub internal: Option<bool>,
    #[serde(rename = "@relation")]
    pub relation: Option<String>,
    // Only present on Event fields.
    #[serde(rename = "@transition")]
    pub transition: Option<TransitionType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XmlType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "@javaType")]
    pub java_type: Option<String>,
    #[serde(rename = "@unsigned")]
    pub unsigned: Option<bool>,
    #[serde(rename = "@parameterType")]
    pub parameter_type: String,
    #[serde(rename = "@fieldType")]
    pub field_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XmlContentType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@annotation")]
    pub annotation: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relation {
    #[serde(rename = "@name")]
    pub name: String,
}
