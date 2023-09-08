// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR event representation.

use crate::{
    error::{Error, Result},
    metadata::{ClassElement, FieldElement},
    resolver::{ConstantResolver, Object, Value},
};

pub struct GenericEvent<'a, 'cr, CR>
where
    CR: ConstantResolver<'a>,
{
    object: Object<'a>,
    constants: &'cr CR,
}

impl<'a, 'cr, CR> GenericEvent<'a, 'cr, CR>
where
    CR: ConstantResolver<'a>,
{
    /// Construct an instance from an owned [Object] and a [ConstantResolver].
    pub fn new(object: Object<'a>, constants: &'cr CR) -> Self {
        Self { object, constants }
    }

    /// Obtain the [ClassElement] being defined.
    pub fn class(&self) -> &ClassElement {
        self.object.class()
    }

    /// Obtain the fully qualified name of the event's class.
    pub fn class_name(&self) -> &str {
        self.object.class().name.as_ref()
    }

    /// Obtain the final part of the class name.
    ///
    /// Whereas [Self::class_name()] evaluates to `jdk.EventPark`, this
    /// function evaluates to `EventPark`.
    pub fn class_name_leaf(&self) -> &str {
        let name = self.object.class().name.as_ref();

        if let Some((_, s)) = name.rsplit_once('.') {
            s
        } else {
            name
        }
    }

    /// Obtain the [FieldElement] at specified index.
    pub fn field_element_at(&self, index: usize) -> Option<&FieldElement> {
        self.object.class().fields.get(index)
    }

    /// Obtain the [Value] for a field at the specified index.
    pub fn field_value_at(&self, index: usize) -> Option<&Value> {
        self.object.field_at(index)
    }

    /// Resolve the [FieldElement] and [Value] for the specified field index.
    pub fn field_at(&self, index: usize) -> Option<(&FieldElement, &Value)> {
        self.object.class().fields.get(index).and_then(|field| {
            self.object
                .field_at(index)
                .and_then(|value| Some((field, value)))
        })
    }

    /// Iterate over [FieldElement] and [Value] pairs.
    pub fn iter_fields(&self) -> impl Iterator<Item = (&FieldElement, &Value)> + '_ {
        self.object
            .class()
            .fields
            .iter()
            .zip(self.object.iter_fields())
    }

    /// Find the field with a given name and return its [FieldElement] and [Value].
    pub fn find_field_named(&self, name: &str) -> Option<(&FieldElement, &Value)> {
        self.iter_fields().find(|(field, _)| field.name == name)
    }

    /// Obtain the raw `startTime` field value.
    ///
    /// This value needs to be combined with metadata in the chunk header
    /// to resolve it to a walk clock time.
    pub fn start_time(&self) -> Result<i64> {
        // The first field for an event should always be `startTime` and should
        // be an i64 primitive.
        let (field, value) = self
            .field_at(0)
            .ok_or_else(|| Error::EventParse("field 0 not found".to_string()))?;

        if field.name != "startTime" {
            return Err(Error::EventParse("field 0 not startTime".to_string()));
        }

        let p = value
            .as_primitive()
            .ok_or_else(|| Error::EventParse("field 0 not a primitive value".to_string()))?;

        let v = p
            .as_long()
            .ok_or_else(|| Error::EventParse("field 0 not an long integer".to_string()))?;

        Ok(v)
    }

    /// Obtain the raw `duration` field value.
    pub fn duration(&self) -> Option<Result<i64>> {
        self.find_field_named("duration").map(|(_, value)| {
            let p = value.as_primitive().ok_or_else(|| {
                Error::EventParse("duration field not a primitive value".to_string())
            })?;

            let v = p.as_long().ok_or_else(|| {
                Error::EventParse("duration field not a long integer".to_string())
            })?;

            Ok(v)
        })
    }

    // TODO eventThread and stackTrace field accessors.
}
