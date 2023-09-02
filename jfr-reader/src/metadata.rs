// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Metadata events.
//!
//! The special metadata event within a chunk defines types encountered
//! within the chunk.
//!
//! The metadata event consists of a header, represented by [MetadataHeader].
//!
//! Following that is a dynamic length string table declaring string data
//! (which can be inline or further referenced in the constants pool). We
//! represent the low-level, lightly parsed string table entries with the
//! [StringRecord] type. These records are fed into a class like
//! [LazyStringTable] to facilitate resolving string data native strings.
//!
//! Following the string table is a tree data structure encoding the types
//! defined by the metadata. Each element in the tree is defined by a generic
//! data structure holding the name of the element, its attribute count,
//! its key-value attributes, and child count. Following that are the
//! element's children. All the names and attributes are integers referring
//! to entries in the string table. We represent these integer-only elements
//! with [ElementRecord].
//!
//! The [MetadataRecords] type holds the results of parsing the integer-domain
//! metadata - all the string table records and the element tree.
//!
//! Entries in the element tree have distinct types with each type having
//! different sets of attributes and different expectations around its child
//! elements. For example, a _field element_ has attributes describing the
//! class ID, name of the field, whether it is an array, and child elements
//! declaring annotations annotations that are added to this field.
//!
//! The various `Raw*Element` structs (e.g. [RawClassElement]) represent the
//! typed versions of [ElementRecord]. The [RawElement] enum does the heavy
//! lifting of converting from [ElementRecord] to the typed variants.
//! [RawElement] still operates in the domain of integers: each element's
//! attributes are still integers referring to entries in the string table.
//!
//! The various `*Element` structs (e.g. [ClassElement]) represent the fully
//! resolved elements, with attributes being resolved into struct fields and
//! their values into appropriate types. (Attribute keys and values are always
//! stored as strings which means that integer attribute values need to be
//! parsed from strings.) The [Element] enum represents a full parsed element,
//! which holds references to fully parsed child elements.
//!
//! The structure of the element tree looks like the following:
//!
//! * [RootElement]
//!   * [MetadataElement]
//!     * [ClassElement] 0..N
//!       * [AnnotationElement] 0..N
//!       * [FieldElement] 0..N
//!         * [AnnotationElement] 0..N
//!       * [SettingsElement] 0..N
//!         * [AnnotationElement] 0..N
//!   * [RegionElement]
//!
//! The most important element is [ClassElement]. It defines a Java type/class.
//!
//! The [Metadata] struct represents the fully parsed metadata event record.
//! It holds a reference to the string table, the [RootElement] at the top of
//! the element tree, and additional fields to facilitate common operations,
//! such as class lookups.

use {
    crate::{
        common::{leb128_i32, leb128_i64},
        error::{Error, ParseResult, Result},
        string_table::{LazyStringTable, StringRecord},
    },
    nom::{error::context, multi::count, sequence::pair},
    std::{borrow::Cow, collections::HashMap, str::FromStr},
};

/// The static header portion of a metadata event.
///
/// All the data up to the dynamic string table data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataHeader {
    pub size: i32,
    /// Should be metadata type id.
    pub event_type_id: i64,
    pub start_time_nanoseconds: i64,
    pub duration_nanoseconds: i64,
    /// Should match the previous ID.
    pub metadata_id: i64,
    /// Number of strings in the string table.
    pub string_count: i32,
}

impl MetadataHeader {
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, size) = leb128_i32(s)?;
        let (s, event_type_id) = leb128_i64(s)?;
        let (s, start_time_nanoseconds) = leb128_i64(s)?;
        let (s, duration_nanoseconds) = leb128_i64(s)?;
        let (s, metadata_id) = leb128_i64(s)?;
        let (s, string_count) = leb128_i32(s)?;

        Ok((
            s,
            Self {
                size,
                event_type_id,
                start_time_nanoseconds,
                duration_nanoseconds,
                metadata_id,
                string_count,
            },
        ))
    }
}

/// A lightly parsed element in a tree.
///
/// Stored values are only integers. Instances are converted into [RawElement] to
/// represent results of parsing and type checking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementRecord {
    /// Index of string in the string table holding the name of this element.
    pub name_index: u32,

    /// Element's attributes.
    ///
    /// Each tuple is a key-value of indices into the string table.
    pub attributes: Vec<(i32, i32)>,

    /// Child elements.
    pub children: Vec<Self>,
}

impl ElementRecord {
    pub fn parse(s: &[u8]) -> ParseResult<Self> {
        let (s, name_index) = leb128_i32(s)?;

        let (s, attribute_count) = leb128_i32(s)?;

        // Each attribute contains pairs of string IDs.
        let (s, attributes) = count(pair(leb128_i32, leb128_i32), attribute_count as usize)(s)?;

        let (s, child_count) = leb128_i32(s)?;

        // Each child is a nested record.
        let (s, children) = context(
            "reading element child records",
            count(Self::parse, child_count as usize),
        )(s)?;

        Ok((
            s,
            Self {
                name_index: name_index as _,
                attributes,
                children,
            },
        ))
    }
}

/// Holds low-level data structures for metadata.
///
/// This is mostly a bag of integers holding the results of lightly parsing the
/// string table and element tree.
#[derive(Clone, Debug)]
pub struct MetadataRecords<'a> {
    header: MetadataHeader,
    string_records: Vec<StringRecord<'a>>,
    root: ElementRecord,
}

impl<'a> MetadataRecords<'a> {
    pub fn parse(s: &'a [u8]) -> ParseResult<Self> {
        let (s, header) = context("parsing metadata event header", MetadataHeader::parse)(s)?;

        let (s, string_records) = context(
            "reading string table records",
            count(StringRecord::parse, header.string_count as usize),
        )(s)?;

        let (s, root) = context("parsing root element record", ElementRecord::parse)(s)?;

        Ok((
            s,
            Self {
                header,
                string_records,
                root,
            },
        ))
    }
}

fn get_str<'a>(st: &mut LazyStringTable<'a>, index: i32) -> Result<Cow<'a, str>> {
    st.get(index as _)?.as_cow().ok_or_else(move || {
        Error::ElementConstructLogic("referenced string does not have inline data".to_string())
    })
}

#[derive(Clone, Debug)]
pub struct RawAnnotationElement {
    pub attributes: Vec<(i32, i32)>,
}

#[derive(Clone, Debug)]
pub struct AnnotationElement<'a> {
    pub type_id: i64,
    pub values: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> AnnotationElement<'a> {
    pub fn from_raw(el: RawAnnotationElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        // There is a single type ID attribute. All others are generic values.
        let mut type_id = None;
        let mut values = Vec::with_capacity(el.attributes.len() - 1);

        for (k, v) in el.attributes {
            let name = get_str(st, k)?;
            let value = get_str(st, v)?;

            match name.as_ref() {
                "class" => {
                    type_id.replace(value);
                }
                _ => {
                    values.push((name, value));
                }
            }
        }

        let type_id = type_id.ok_or_else(|| {
            Error::ElementConstructLogic("annotation lacks type id attribute".to_string())
        })?;
        let type_id = i64::from_str(type_id.as_ref()).map_err(|e| {
            Error::ElementConstructLogic(format!("error parsing annotation class id to int: {}", e))
        })?;

        Ok(Self { type_id, values })
    }
}

#[derive(Clone, Debug)]
pub struct RawClassElement {
    pub annotations: Vec<RawAnnotationElement>,
    pub fields: Vec<RawFieldElement>,
    pub settings: Vec<RawSettingsElement>,
    pub attributes: Vec<(i32, i32)>,
}

/// Defines a Java class/type.
#[derive(Clone, Debug)]
pub struct ClassElement<'a> {
    /// Holds any further annotations that can be present on instances of this class.
    pub annotations: Vec<AnnotationElement<'a>>,
    /// Describes the fields of instances of this type.
    pub fields: Vec<FieldElement<'a>>,
    /// Describes any settings related to this type.
    pub settings: Vec<SettingsElement<'a>>,
    /// The name of the class. e.g. `java.lang.String`.
    pub name: Cow<'a, str>,
    /// The super/parent class name.
    pub super_type: Option<Cow<'a, str>>,
    /// Whether this is a primitive / built-in type.
    pub simple_type: Option<Cow<'a, str>>,
    /// The class ID used to refer to this class.
    pub id: i64,
}

impl<'a> ClassElement<'a> {
    pub fn from_raw(el: RawClassElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        let annotations = el
            .annotations
            .into_iter()
            .map(|a| AnnotationElement::from_raw(a, st))
            .collect::<Result<Vec<_>>>()?;
        let fields = el
            .fields
            .into_iter()
            .map(|f| FieldElement::from_raw(f, st))
            .collect::<Result<Vec<_>>>()?;
        let settings = el
            .settings
            .into_iter()
            .map(|s| SettingsElement::from_raw(s, st))
            .collect::<Result<Vec<_>>>()?;

        let mut name = None;
        let mut super_type = None;
        let mut simple_type = None;
        let mut id = None;

        for (k, v) in el.attributes {
            let k = get_str(st, k)?;
            let v = get_str(st, v)?;

            match k.as_ref() {
                "name" => {
                    name.replace(v);
                }
                "superType" => {
                    super_type.replace(v);
                }
                "simpleType" => {
                    simple_type.replace(v);
                }
                "id" => {
                    id.replace(v);
                }
                name => {
                    return Err(Error::ElementConstructLogic(format!(
                        "class element has unexpected attribute: {}",
                        name
                    )));
                }
            }
        }

        let name = name.ok_or_else(|| {
            Error::ElementConstructLogic("class lacks name attribute".to_string())
        })?;
        let id =
            id.ok_or_else(|| Error::ElementConstructLogic("class lacks id attribute".to_string()))?;
        let id = i64::from_str(id.as_ref()).map_err(|e| {
            Error::ElementConstructLogic(format!(
                "class element id fails to parse as integer: {}",
                e
            ))
        })?;

        Ok(Self {
            annotations,
            fields,
            settings,
            name,
            super_type,
            simple_type,
            id,
        })
    }

    /// Obtain an iterable of all referenced annotations in this class definition.
    ///
    /// Resolves the annotations for the class, fields, and settings. There may
    /// be duplicate entries in the stream.
    pub fn all_annotations(&self) -> impl Iterator<Item = &AnnotationElement<'a>> + '_ {
        self.annotations
            .iter()
            .chain(self.fields.iter().flat_map(|f| f.annotations.iter()))
            .chain(self.settings.iter().flat_map(|s| s.annotations.iter()))
    }
}

#[derive(Clone, Debug)]
pub struct RawFieldElement {
    pub annotations: Vec<RawAnnotationElement>,
    pub attributes: Vec<(i32, i32)>,
}

/// Describes a field in a class / type.
#[derive(Clone, Debug)]
pub struct FieldElement<'a> {
    /// Additional annotations for this field.
    pub annotations: Vec<AnnotationElement<'a>>,
    /// The name of this field.
    pub name: Cow<'a, str>,
    /// The class id / type of this field.
    pub type_id: i64,
    /// Number of dimensions. A value > 0 means this is an array.
    pub dimension: Option<i64>,
    /// The value for the field is a LEB-128 reference to a constant pool index
    /// for this field's class/type ID instead of an inline value.
    pub constant_pool: Option<Cow<'a, str>>,
}

impl<'a> FieldElement<'a> {
    pub fn from_raw(el: RawFieldElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        let annotations = el
            .annotations
            .into_iter()
            .map(|a| AnnotationElement::from_raw(a, st))
            .collect::<Result<Vec<_>>>()?;

        let mut name = None;
        let mut type_id = None;
        let mut dimension = None;
        let mut constant_pool = None;

        for (k, v) in el.attributes {
            let k = get_str(st, k)?;
            let v = get_str(st, v)?;

            match k.as_ref() {
                "name" => {
                    name.replace(v);
                }
                "class" => {
                    type_id.replace(v);
                }
                "dimension" => {
                    dimension.replace(v);
                }
                "constantPool" => {
                    constant_pool.replace(v);
                }
                name => {
                    return Err(Error::ElementConstructLogic(format!(
                        "field element has unexpected attribute: {}",
                        name
                    )));
                }
            }
        }

        let name = name.ok_or_else(|| {
            Error::ElementConstructLogic("field lacks name attribute".to_string())
        })?;
        let type_id = type_id.ok_or_else(|| {
            Error::ElementConstructLogic("field lacks class attribute".to_string())
        })?;
        let type_id = i64::from_str(type_id.as_ref()).map_err(|e| {
            Error::ElementConstructLogic(format!(
                "failed to parse field element class id to integer: {}",
                e
            ))
        })?;
        let dimension = if let Some(x) = dimension {
            Some(i64::from_str(x.as_ref()).map_err(|e| {
                Error::ElementConstructLogic(format!(
                    "field element dimension could not be parsed to integer: {}",
                    e
                ))
            })?)
        } else {
            None
        };

        Ok(Self {
            annotations,
            name,
            type_id,
            dimension,
            constant_pool,
        })
    }

    /// Whether this field is an array.
    pub fn is_array_type(&self) -> bool {
        self.dimension.unwrap_or(0) > 0
    }
}

#[derive(Clone, Debug)]
pub struct RawMetadataElement {
    pub classes: Vec<RawClassElement>,
    pub attributes: Vec<(i32, i32)>,
}

/// Describes chunk metadata.
#[derive(Clone, Debug)]
pub struct MetadataElement<'a> {
    /// The classes/types this chunk references.
    pub classes: Vec<ClassElement<'a>>,
}

impl<'a> MetadataElement<'a> {
    pub fn from_raw(el: RawMetadataElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        let classes = el
            .classes
            .into_iter()
            .map(|cls| ClassElement::from_raw(cls, st))
            .collect::<Result<Vec<_>>>()?;

        if !el.attributes.is_empty() {
            return Err(Error::ElementConstructLogic(
                "metadata has attributes unexpectedly".to_string(),
            ));
        }

        Ok(Self { classes })
    }
}

#[derive(Clone, Debug)]
pub struct RawRegionElement {
    pub attributes: Vec<(i32, i32)>,
}

/// Encodes information about the locale / region from whence this chunk came.
#[derive(Clone, Debug)]
pub struct RegionElement<'a> {
    pub locale: Cow<'a, str>,
    pub gmt_offset: Cow<'a, str>,
}

impl<'a> RegionElement<'a> {
    pub fn from_raw(el: RawRegionElement, string_table: &mut LazyStringTable<'a>) -> Result<Self> {
        let mut locale = None;
        let mut gmt_offset = None;

        for (k, v) in el.attributes {
            let k = get_str(string_table, k)?;
            let v = get_str(string_table, v)?;

            match k.as_ref() {
                "locale" => {
                    locale.replace(v);
                }
                "gmtOffset" => {
                    gmt_offset.replace(v);
                }
                name => {
                    return Err(Error::ElementConstructLogic(format!(
                        "region element has unexpected attribute: {}",
                        name
                    )));
                }
            }
        }

        let locale = locale.ok_or_else(|| {
            Error::ElementConstructLogic("region lacks locale attribute".to_string())
        })?;
        let gmt_offset = gmt_offset.ok_or_else(|| {
            Error::ElementConstructLogic("region lacks gmtOffset attribute".to_string())
        })?;

        Ok(Self { locale, gmt_offset })
    }
}

#[derive(Clone, Debug)]
pub struct RawRootElement {
    pub metadata: RawMetadataElement,
    pub region: RawRegionElement,
    pub attributes: Vec<(i32, i32)>,
}

/// The root element in the metadata element tree.
///
/// This element exists to hold the other top-level elements.
#[derive(Clone, Debug)]
pub struct RootElement<'a> {
    pub metadata: MetadataElement<'a>,
    pub region: RegionElement<'a>,
}

impl<'a> RootElement<'a> {
    pub fn from_raw(el: RawRootElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        let metadata = MetadataElement::from_raw(el.metadata, st)?;
        let region = RegionElement::from_raw(el.region, st)?;

        if !el.attributes.is_empty() {
            return Err(Error::ElementConstructLogic(
                "root element has attributes unexpectedly".to_string(),
            ));
        }

        Ok(Self { metadata, region })
    }
}

#[derive(Clone, Debug)]
pub struct RawSettingsElement {
    pub annotations: Vec<RawAnnotationElement>,
    pub attributes: Vec<(i32, i32)>,
}

/// Settings related to a class/type.
#[derive(Clone, Debug)]
pub struct SettingsElement<'a> {
    /// Additional annotations for these settings.
    pub annotations: Vec<AnnotationElement<'a>>,
    /// The name of the setting.
    pub name: Cow<'a, str>,
    /// The class / type ID for the value of this setting.
    pub type_id: i64,
    /// The default value for this setting.
    pub default_value: Cow<'a, str>,
}

impl<'a> SettingsElement<'a> {
    pub fn from_raw(el: RawSettingsElement, st: &mut LazyStringTable<'a>) -> Result<Self> {
        let annotations = el
            .annotations
            .into_iter()
            .map(|a| AnnotationElement::from_raw(a, st))
            .collect::<Result<Vec<_>>>()?;

        let mut name = None;
        let mut type_id = None;
        let mut default_value = None;

        for (k, v) in el.attributes {
            let k = get_str(st, k)?;
            let v = get_str(st, v)?;

            match k.as_ref() {
                "name" => {
                    name.replace(v);
                }
                "class" => {
                    type_id.replace(v);
                }
                "defaultValue" => {
                    default_value.replace(v);
                }
                name => {
                    return Err(Error::ElementConstructLogic(format!(
                        "setting element has unexpected attribute: {}",
                        name
                    )));
                }
            }
        }

        let name = name.ok_or_else(|| {
            Error::ElementConstructLogic("setting lacks name attribute".to_string())
        })?;
        let type_id = type_id.ok_or_else(|| {
            Error::ElementConstructLogic("setting lacks class attribute".to_string())
        })?;
        let type_id = i64::from_str(type_id.as_ref()).map_err(|e| {
            Error::ElementConstructLogic(format!(
                "setting element failed to parse class id to integer: {}",
                e
            ))
        })?;
        let default_value = default_value.ok_or_else(|| {
            Error::ElementConstructLogic("setting lacks defaultValue attribute".to_string())
        })?;

        Ok(Self {
            annotations,
            name,
            type_id,
            default_value,
        })
    }
}

/// A parsed element and its children elements but without parsed attribute data.
///
/// This type exists to facilitate avoiding overhead with parsing attributes on
/// elements we don't care about.
#[derive(Clone, Debug)]
pub enum RawElement {
    Annotation(RawAnnotationElement),
    Class(RawClassElement),
    Field(RawFieldElement),
    Metadata(RawMetadataElement),
    Region(RawRegionElement),
    Root(RawRootElement),
    Setting(RawSettingsElement),
}

impl RawElement {
    /// Construct an instance from its lightly parsed [ElementRecord].
    ///
    /// Uses a string table for name lookups.
    pub fn from_record(record: ElementRecord, string_table: &mut LazyStringTable) -> Result<Self> {
        // Children are non-lazily constructed as well.
        let children = record
            .children
            .into_iter()
            .map(|child| Self::from_record(child, string_table))
            .collect::<Result<Vec<_>>>()?;

        let name = string_table.get(record.name_index as _)?;

        // String references in metadata can never reference the constants pool because
        // it would be a circular dependency. So we must be able to resolve a string
        // here.
        let name = name.as_str().ok_or(Error::ElementConstructLogic(
            "referenced string does not have inline data; did you find a logic bug?".to_string(),
        ))?;

        match name {
            "annotation" => Ok(Self::Annotation(RawAnnotationElement {
                attributes: record.attributes,
            })),
            "class" => {
                let mut annotations = vec![];
                let mut fields = vec![];
                let mut settings = vec![];

                for el in children {
                    match el {
                        RawElement::Annotation(a) => {
                            annotations.push(a);
                        }
                        RawElement::Field(f) => {
                            fields.push(f);
                        }
                        RawElement::Setting(s) => {
                            settings.push(s);
                        }
                        _ => {
                            return Err(Error::ElementConstructLogic(format!(
                                "unexpected {} element in class element",
                                el.name()
                            )));
                        }
                    }
                }

                Ok(Self::Class(RawClassElement {
                    annotations,
                    fields,
                    settings,
                    attributes: record.attributes,
                }))
            }
            "field" => {
                // All children elements should be annotations.
                let annotations = children
                    .into_iter()
                    .map(|el| {
                        if let RawElement::Annotation(a) = el {
                            Ok(a)
                        } else {
                            Err(Error::ElementConstructLogic(format!(
                                "unexpected non-annotation element in field: {}",
                                el.name()
                            )))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(Self::Field(RawFieldElement {
                    annotations,
                    attributes: record.attributes,
                }))
            }
            "metadata" => {
                let classes = children
                    .into_iter()
                    .map(|el| {
                        if let RawElement::Class(class) = el {
                            Ok(class)
                        } else {
                            Err(Error::ElementConstructLogic(format!(
                                "unexpected non-class element in metadata: {}",
                                el.name()
                            )))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(Self::Metadata(RawMetadataElement {
                    classes,
                    attributes: record.attributes,
                }))
            }
            "region" => Ok(Self::Region(RawRegionElement {
                attributes: record.attributes,
            })),
            "root" => {
                let mut metadata = None;
                let mut region = None;

                for el in children {
                    match el {
                        RawElement::Metadata(x) => {
                            metadata.replace(x);
                        }
                        RawElement::Region(x) => {
                            region.replace(x);
                        }
                        _ => {
                            return Err(Error::ElementConstructLogic(format!(
                                "unexpected child element {} in root",
                                el.name()
                            )));
                        }
                    }
                }

                let metadata = metadata.ok_or_else(|| {
                    Error::ElementConstructLogic("root element lacks metadata element".to_string())
                })?;
                let region = region.ok_or_else(|| {
                    Error::ElementConstructLogic("root element lacks region element".to_string())
                })?;

                Ok(Self::Root(RawRootElement {
                    metadata,
                    region,
                    attributes: record.attributes,
                }))
            }
            "setting" => {
                // All children elements should be annotations.
                let annotations = children
                    .into_iter()
                    .map(|el| {
                        if let RawElement::Annotation(a) = el {
                            Ok(a)
                        } else {
                            Err(Error::ElementConstructLogic(format!(
                                "unexpected non-annotation element in setting: {}",
                                el.name()
                            )))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(Self::Setting(RawSettingsElement {
                    annotations,
                    attributes: record.attributes,
                }))
            }

            _ => Err(Error::ElementNameUnknown(name.to_string())),
        }
    }

    /// The name of this element variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Annotation(_) => "annotation",
            Self::Class(_) => "class",
            Self::Metadata(_) => "metadata",
            Self::Field(_) => "field",
            Self::Region(_) => "region",
            Self::Root(_) => "root",
            Self::Setting(_) => "setting",
        }
    }

    /// Resolve attributes string table keys and values.
    pub fn attributes(&self) -> &[(i32, i32)] {
        match self {
            Self::Annotation(x) => &x.attributes,
            RawElement::Class(x) => &x.attributes,
            RawElement::Field(x) => &x.attributes,
            RawElement::Metadata(x) => &x.attributes,
            RawElement::Region(x) => &x.attributes,
            RawElement::Root(x) => &x.attributes,
            RawElement::Setting(x) => &x.attributes,
        }
    }
}

/// A fully parsed element.
///
/// The element, its attributes, its children, and all its attributes
/// are fully parsed. All element data is available.
#[derive(Clone, Debug)]
pub enum Element<'a> {
    Annotation(AnnotationElement<'a>),
    Class(ClassElement<'a>),
    Field(FieldElement<'a>),
    Metadata(MetadataElement<'a>),
    Region(RegionElement<'a>),
    Root(RootElement<'a>),
    Setting(SettingsElement<'a>),
}

impl<'a> Element<'a> {
    /// Construct an instance from a [RawElement].
    ///
    /// This effectively parses attributes and performs type checking against
    /// attributes.
    ///
    /// Parsing operates recursively to child elements.
    pub fn from_raw<'st>(el: RawElement, st: &'st mut LazyStringTable<'a>) -> Result<Self> {
        match el {
            RawElement::Annotation(x) => Ok(Self::Annotation(AnnotationElement::from_raw(x, st)?)),
            RawElement::Class(x) => Ok(Self::Class(ClassElement::from_raw(x, st)?)),
            RawElement::Field(x) => Ok(Self::Field(FieldElement::from_raw(x, st)?)),
            RawElement::Metadata(x) => Ok(Self::Metadata(MetadataElement::from_raw(x, st)?)),
            RawElement::Region(x) => Ok(Self::Region(RegionElement::from_raw(x, st)?)),
            RawElement::Root(x) => Ok(Self::Root(RootElement::from_raw(x, st)?)),
            RawElement::Setting(x) => Ok(Self::Setting(SettingsElement::from_raw(x, st)?)),
        }
    }
}

/// Describes types encountered within a chunk.
///
/// This represents the results of parsing a metadata event and decoding its
/// contents.
///
/// The metadata event holds a string table defining, well, string data. It
/// also holds the definition of all classes/types encountered in the chunk.
///
/// Instances of this type are used to resolve class/type information from
/// integer references to class IDs.
pub struct Metadata<'a> {
    /// The metadata header from whence we came.
    pub header: MetadataHeader,

    /// The string table within the event data.
    pub string_table: LazyStringTable<'a>,

    /// The root element in the element tree.
    pub root: RootElement<'a>,

    /// A mapping of class / type IDs to class elements describing them.
    pub class_map: HashMap<i64, ClassElement<'a>>,
}

impl<'a> Metadata<'a> {
    /// Construct an instance from metadata event data.
    ///
    /// Input should be the beginning of a chunk event record. The size and event
    /// type will be parsed.
    pub fn parse(s: &'a [u8]) -> Result<(&'a [u8], Self)> {
        let (s, records) = MetadataRecords::parse(s)?;

        let metadata = Self::from_records(records)?;

        Ok((s, metadata))
    }

    /// Convert [MetadataRecords] into an instance of this type.
    ///
    /// This performs the work of resolving integer references to strings and
    /// other typed values. It also constructs the class map.
    pub fn from_records(records: MetadataRecords<'a>) -> Result<Self> {
        let mut string_table = LazyStringTable::from(records.string_records);

        // Cast to typed elements.
        let root = RawElement::from_record(records.root, &mut string_table)?;

        let root = Element::from_raw(root, &mut string_table)?;

        let root = if let Element::Root(x) = root {
            x
        } else {
            return Err(Error::ElementConstructLogic(
                "metadata root element has wrong type".to_string(),
            ));
        };

        // TODO consider using a sparse vec since class IDs appear to be mostly
        // sequential. Incurring a key hash seems like avoidable overhead.
        let mut class_map = HashMap::with_capacity(root.metadata.classes.len());

        for class in root.metadata.classes.iter() {
            class_map.insert(class.id, class.clone());
        }

        Ok(Self {
            header: records.header,
            string_table,
            root,
            class_map,
        })
    }
}
