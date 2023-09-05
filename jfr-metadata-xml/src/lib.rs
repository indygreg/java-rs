// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod schema;
pub use schema::Metadata;

use crate::schema::{Event, RootElement, Type, XmlContentType, XmlType};
use quick_xml::de::DeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("deserialization: {0}")]
    De(#[from] DeError),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

impl Metadata {
    /// Attempt to deserialize an instance from a string.
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(quick_xml::de::from_str(s)?)
    }

    /// Attempt to deserialize an instance from a reader.
    pub fn from_reader(reader: impl std::io::BufRead) -> Result<Self> {
        Ok(quick_xml::de::from_reader(reader)?)
    }

    /// Attempt to load XML from a filesystem path.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let fh = std::io::BufReader::new(std::fs::File::open(path)?);

        Self::from_reader(fh)
    }

    /// Obtain all [Event] in this metadata.
    pub fn events(&self) -> impl Iterator<Item = &Event> + '_ {
        self.elements.iter().filter_map(|e| {
            if let RootElement::Event(x) = e {
                Some(x)
            } else {
                None
            }
        })
    }

    /// Obtain all [Type] in this metadata.
    pub fn types(&self) -> impl Iterator<Item = &Type> + '_ {
        self.elements.iter().filter_map(|e| {
            if let RootElement::Type(x) = e {
                Some(x)
            } else {
                None
            }
        })
    }

    /// Obtain all [XmlType] in this metadata.
    pub fn xml_types(&self) -> impl Iterator<Item = &XmlType> + '_ {
        self.elements.iter().filter_map(|e| {
            if let RootElement::XmlType(x) = e {
                Some(x)
            } else {
                None
            }
        })
    }

    /// Obtain all [XmlContentType] in this metdata.
    pub fn xml_content_types(&self) -> impl Iterator<Item = &XmlContentType> + '_ {
        self.elements.iter().filter_map(|e| {
            if let RootElement::XmlContentType(x) = e {
                Some(x)
            } else {
                None
            }
        })
    }

    /// Find the [Type] having the specified name.
    pub fn find_type(&self, name: &str) -> Option<&Type> {
        self.types().find(|t| t.name == name)
    }

    /// Find the [XmlType] having the specified name.
    pub fn find_xml_type(&self, name: &str) -> Option<&XmlType> {
        self.xml_types().find(|t| t.name == name)
    }

    /// Obtain a sorted vector of events.
    pub fn events_sorted(&self) -> Vec<&Event> {
        let mut events = self.events().collect::<Vec<_>>();
        events.sort_by_key(|x| x.name.as_str());

        events
    }

    /// Obtain a sorted vector of types.
    pub fn types_sorted(&self) -> Vec<&Type> {
        let mut types = self.types().collect::<Vec<_>>();
        types.sort_by_key(|x| x.name.as_str());

        types
    }
}
