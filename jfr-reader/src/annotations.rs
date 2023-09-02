// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR Annotations.
//!
//! This module attempts to define the well-known annotations present
//! and emitted by common JVM implementations.
//!
//! Some of these annotations are emitted in chunk metadata as
//! [AnnotationElement]. Some are internal to JFR and are only used to
//! describe events or their behavior.

use crate::{
    error::{Error, Result},
    metadata::{AnnotationElement, ClassElement},
};
use std::{borrow::Cow, ops::Deref, str::FromStr};

/// What type of elements an annotation applies to.
pub enum Target {
    /// An annotation.
    Annotation,
    /// A class / type.
    Class,
    /// A field.
    Field,
    /// A method.
    Method,
}

/// Trait describing an annotation.
pub trait Annotation: Sized {
    /// The name of the annotation's class.
    fn class_name() -> &'static str;

    /// The type of elements that the annotation can be attached to.
    fn targets() -> Vec<Target>;
}

/// A `jdk.jfr.BooleanFlag` annotation.
#[derive(Clone, Debug)]
pub struct BooleanFlag(pub bool);

impl Deref for BooleanFlag {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Annotation for BooleanFlag {
    fn class_name() -> &'static str {
        "jdk.jfr.BooleanFlag"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl BooleanFlag {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        // Values appear to always be empty.
        Ok(Self(true))
    }
}

/// A `jdk.jfr.Category` annotation.
///
/// Just a wrapper to facilitate code reuse.
#[derive(Clone, Debug)]
pub struct Category<'a>(pub Vec<Cow<'a, str>>);

impl<'a> Deref for Category<'a> {
    type Target = Vec<Cow<'a, str>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Annotation for Category<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Category"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl<'a> Category<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let values = el.values.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();

        Ok(Self(values))
    }
}

/// A `jdk.jfr.ContentType` annotation.
#[derive(Clone, Debug)]
pub struct ContentType {}

impl Annotation for ContentType {
    fn class_name() -> &'static str {
        "jdk.jfr.ContentType"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Annotation]
    }
}

impl ContentType {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// Describes an amount of data.
///
/// Represents a `jdk.jfr.DataAmount` instance.
#[derive(Clone, Debug)]
pub enum DataAmount<'a> {
    Bits,
    Bytes,
    Other(Cow<'a, str>),
}

impl<'a> Annotation for DataAmount<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.DataAmount"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> DataAmount<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        // Should only be a single value.
        let (_, value) = el.values.first().ok_or_else(|| {
            Error::AnnotationParse("jdk.jfr.DataAmount lacks a value".to_string())
        })?;

        match value.as_ref() {
            "BITS" => Ok(Self::Bits),
            "BYTES" => Ok(Self::Bytes),
            _ => Ok(Self::Other(value.clone())),
        }
    }
}

/// A `jdk.jfr.Description` annotation.
#[derive(Clone, Debug)]
pub struct Description<'a>(pub Cow<'a, str>);

impl<'a> Deref for Description<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Annotation for Description<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Description"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> Description<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el.values.first().ok_or_else(|| {
            Error::AnnotationParse("jdk.jfr.Description lacks a value".to_string())
        })?;

        Ok(Self(value.clone()))
    }
}

/// A `jdk.jfr.Enabled` annotation.
#[derive(Clone, Debug)]
pub struct Enabled(pub bool);

impl Deref for Enabled {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Annotation for Enabled {
    fn class_name() -> &'static str {
        "jdk.jfr.Enabled"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl Enabled {
    fn from_element(el: &AnnotationElement) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Enabled lacks a value".to_string()))?;

        match value.as_ref() {
            "true" => Ok(Self(true)),
            "false" => Ok(Self(false)),
            _ => Err(Error::AnnotationParse(format!(
                "jdk.jfr.Enabled illegal value: {value}"
            ))),
        }
    }
}

/// A `jdk.jfr.Enabled` annotation.
#[derive(Clone, Debug)]
pub struct Experimental(pub bool);

impl Deref for Experimental {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Annotation for Experimental {
    fn class_name() -> &'static str {
        "jdk.jfr.Experimental"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field]
    }
}

impl Experimental {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        // If this appears the values are empty and it is implicit true.
        Ok(Self(true))
    }
}

/// The value is a frequency measured in Hertz.
#[derive(Clone, Debug)]
pub struct Frequency {}

impl Annotation for Frequency {
    fn class_name() -> &'static str {
        "jdk.jfr.Frequency"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Field, Target::Method]
    }
}

impl Frequency {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// A `jdk.jfr.Label` annotation.
///
/// Just a wrapper to facilitate code reuse.
#[derive(Clone, Debug)]
pub struct Label<'a>(Cow<'a, str>);

impl<'a> Deref for Label<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Annotation for Label<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Label"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> Label<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Label lacks a value".to_string()))?;

        Ok(Self(value.clone()))
    }
}

/// The value is a memory address.
#[derive(Clone, Debug)]
pub struct MemoryAddress {}

impl Annotation for MemoryAddress {
    fn class_name() -> &'static str {
        "jdk.jfr.MemoryAddress"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl MemoryAddress {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// Defines new types of event metadata.
#[derive(Clone, Debug)]
pub struct MetadataDefinition {}

impl Annotation for MetadataDefinition {
    fn class_name() -> &'static str {
        "jdk.jfr.MetadataDefinition"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl MetadataDefinition {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// A `jdk.jfr.Name` annotation.
///
/// Just a wrapper to facilitate code reuse.
#[derive(Clone, Debug)]
pub struct Name<'a>(Cow<'a, str>);

impl<'a> Deref for Name<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Annotation for Name<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Name"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> Name<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Name lacks a value".to_string()))?;

        Ok(Self(value.clone()))
    }
}

/// Annotation to be used on fractions representing percentages.
#[derive(Clone, Debug)]
pub struct Percentage {}

impl Annotation for Percentage {
    fn class_name() -> &'static str {
        "jdk.jfr.Percentage"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl Percentage {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// Default setting for a periodic event.
#[derive(Clone, Debug)]
pub enum Period<'a> {
    // TODO there are other special values on the settings equivalent. Shouldn't they be here?
    EveryChunk,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Period<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Period"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl<'a> Period<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Period lacks value".to_string()))?;

        if value == "everyChunk" {
            Ok(Self::EveryChunk)
        } else {
            if let Some((number, unit)) = value.split_once(' ') {
                let number = u64::from_str(number).map_err(|e| {
                    Error::AnnotationParse(format!("jdk.jfr.Period failed to parse integer: {e}"))
                })?;

                match unit {
                    "ns" => Ok(Self::Nanoseconds(number)),
                    "us" => Ok(Self::Microseconds(number)),
                    "ms" => Ok(Self::Milliseconds(number)),
                    "s" => Ok(Self::Seconds(number)),
                    "m" => Ok(Self::Minutes(number)),
                    "h" => Ok(Self::Hours(number)),
                    "d" => Ok(Self::Days(number)),
                    _ => Err(Error::AnnotationParse(format!(
                        "jdk.jfr.Period has invalid unit: {unit}"
                    ))),
                }
            } else {
                Ok(Self::Other(value.clone()))
            }
        }
    }
}

/// A `jdk.jfr.Registered` annotation.
#[derive(Clone, Debug)]
pub struct Registered(pub bool);

impl Deref for Registered {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Annotation for Registered {
    fn class_name() -> &'static str {
        "jdk.jfr.Registered"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl Registered {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self(true))
    }
}

/// A `jdk.jfr.Relational` annotation.
#[derive(Clone, Debug)]
pub struct Relational {}

impl Annotation for Relational {
    fn class_name() -> &'static str {
        "jdk.jfr.Relational"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Annotation]
    }
}

impl Relational {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// A `jdk.jfr.SettingDefinition` annotation.
#[derive(Clone, Debug)]
pub struct SettingDefinition {}

impl Annotation for SettingDefinition {
    fn class_name() -> &'static str {
        "jdk.jfr.SettingDefinition"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Method]
    }
}

impl SettingDefinition {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// A `jdk.jfr.StackTrace` annotation.
#[derive(Clone, Debug)]
pub struct StackTrace(pub bool);

impl Deref for StackTrace {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Annotation for StackTrace {
    fn class_name() -> &'static str {
        "jdk.jfr.StackTrace"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl StackTrace {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self(true))
    }
}

#[derive(Clone, Debug)]
pub enum Threshold<'a> {
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Threshold<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Threshold"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl<'a> Threshold<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Threshold lacks value".to_string()))?;

        if let Some((number, unit)) = value.split_once(' ') {
            let number = u64::from_str(number).map_err(|e| {
                Error::AnnotationParse(format!("jdk.jfr.Threshold failed to parse integer: {e}"))
            })?;

            match unit {
                "ns" => Ok(Self::Nanoseconds(number)),
                "us" => Ok(Self::Microseconds(number)),
                "ms" => Ok(Self::Milliseconds(number)),
                "s" => Ok(Self::Seconds(number)),
                "m" => Ok(Self::Minutes(number)),
                "h" => Ok(Self::Hours(number)),
                "d" => Ok(Self::Days(number)),
                _ => Err(Error::AnnotationParse(format!(
                    "jdk.jfr.Threshold has invalid unit: {unit}"
                ))),
            }
        } else {
            Ok(Self::Other(value.clone()))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Timespan<'a> {
    Ticks,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Timespan<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Timespan"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> Timespan<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Timespan lacks value".to_string()))?;

        match value.as_ref() {
            "TICKS" => Ok(Self::Ticks),
            "SECONDS" => Ok(Self::Seconds),
            "MILLISECONDS" => Ok(Self::Milliseconds),
            "MICROSECONDS" => Ok(Self::Microseconds),
            "NANOSECONDS" => Ok(Self::Nanoseconds),
            _ => Ok(Self::Other(value.clone())),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Timestamp<'a> {
    MillisecondsSinceEpoch,
    Ticks,
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Timestamp<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.Timestamp"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field, Target::Method]
    }
}

impl<'a> Timestamp<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el
            .values
            .first()
            .ok_or_else(|| Error::AnnotationParse("jdk.jfr.Timestamp lacks value".to_string()))?;

        match value.as_ref() {
            "MILLISECONDS_SINCE_EPOCH" => Ok(Self::MillisecondsSinceEpoch),
            "TICKS" => Ok(Self::Ticks),
            _ => Ok(Self::Other(value.clone())),
        }
    }
}

/// The event transitioned from a thread.
#[derive(Clone, Debug)]
pub struct TransitionFrom {}

impl Annotation for TransitionFrom {
    fn class_name() -> &'static str {
        "jdk.jfr.TransitionFrom"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Field]
    }
}

impl TransitionFrom {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// The event will transition to a thread.
#[derive(Clone, Debug)]
pub struct TransitionTo {}

impl Annotation for TransitionTo {
    fn class_name() -> &'static str {
        "jdk.jfr.TransitionTo"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Field]
    }
}

impl TransitionTo {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

/// Value is an unsigned data type.
#[derive(Clone, Debug)]
pub struct Unsigned {}

impl Annotation for Unsigned {
    fn class_name() -> &'static str {
        "jdk.jfr.Unsigned"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class, Target::Field]
    }
}

impl Unsigned {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub enum Cutoff<'a> {
    Infinity,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Cutoff<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.internal.Cutoff"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl<'a> Cutoff<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el.values.first().ok_or_else(|| {
            Error::AnnotationParse("jdk.jfr.internal.Cutoff lacks value".to_string())
        })?;

        if value == "infinity" {
            Ok(Self::Infinity)
        } else {
            if let Some((number, unit)) = value.split_once(' ') {
                let number = u64::from_str(number).map_err(|e| {
                    Error::AnnotationParse(format!(
                        "jdk.jfr.internal.Cutoff failed to parse integer: {e}"
                    ))
                })?;

                match unit {
                    "ns" => Ok(Self::Nanoseconds(number)),
                    "us" => Ok(Self::Microseconds(number)),
                    "ms" => Ok(Self::Milliseconds(number)),
                    "s" => Ok(Self::Seconds(number)),
                    "m" => Ok(Self::Minutes(number)),
                    "h" => Ok(Self::Hours(number)),
                    "d" => Ok(Self::Days(number)),
                    _ => Err(Error::AnnotationParse(format!(
                        "jdk.jfr.internal.Cutoff has invalid unit: {unit}"
                    ))),
                }
            } else {
                Ok(Self::Other(value.clone()))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mirror {}

impl Annotation for Mirror {
    fn class_name() -> &'static str {
        "jdk.jfr.internal.Mirror"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl Mirror {
    fn from_element(_: &AnnotationElement) -> Result<Self> {
        Ok(Self {})
    }
}

#[derive(Clone, Debug)]
pub enum Throttle<'a> {
    Off,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Other(Cow<'a, str>),
}

impl<'a> Annotation for Throttle<'a> {
    fn class_name() -> &'static str {
        "jdk.jfr.internal.Throttle"
    }

    fn targets() -> Vec<Target> {
        vec![Target::Class]
    }
}

impl<'a> Throttle<'a> {
    fn from_element(el: &AnnotationElement<'a>) -> Result<Self> {
        let (_, value) = el.values.first().ok_or_else(|| {
            Error::AnnotationParse("jdk.jfr.internal.Throttle lacks value".to_string())
        })?;

        if value == "off" {
            Ok(Self::Off)
        } else {
            if let Some((number, unit)) = value.split_once('/') {
                let number = u64::from_str(number).map_err(|e| {
                    Error::AnnotationParse(format!(
                        "jdk.jfr.internal.Throttle failed to parse integer: {e}"
                    ))
                })?;

                match unit {
                    "ns" => Ok(Self::Nanoseconds(number)),
                    "us" => Ok(Self::Microseconds(number)),
                    "ms" => Ok(Self::Milliseconds(number)),
                    "s" => Ok(Self::Seconds(number)),
                    "m" => Ok(Self::Minutes(number)),
                    "h" => Ok(Self::Hours(number)),
                    "d" => Ok(Self::Days(number)),
                    _ => Err(Error::AnnotationParse(format!(
                        "jdk.jfr.internal.Cutoff has invalid unit: {unit}"
                    ))),
                }
            } else {
                Ok(Self::Other(value.clone()))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnnotationValue<'a> {
    BooleanFlag(BooleanFlag),
    Category(Category<'a>),
    ContentType(ContentType),
    DataAmount(DataAmount<'a>),
    Description(Description<'a>),
    Enabled(Enabled),
    Experimental(Experimental),
    Frequency(Frequency),
    Label(Label<'a>),
    MemoryAddress(MemoryAddress),
    MetadataDefinition(MetadataDefinition),
    Name(Name<'a>),
    Percentage(Percentage),
    Period(Period<'a>),
    Registered(Registered),
    Relational(Relational),
    SettingDefinition(SettingDefinition),
    StackTrace(StackTrace),
    Threshold(Threshold<'a>),
    Timespan(Timespan<'a>),
    Timestamp(Timestamp<'a>),
    TransitionFrom(TransitionFrom),
    TransitionTo(TransitionTo),
    Unsigned(Unsigned),
    Cutoff(Cutoff<'a>),
    Mirror(Mirror),
    Throttle(Throttle<'a>),
    Unknown(Vec<(Cow<'a, str>, Cow<'a, str>)>),
}

impl<'a> AnnotationValue<'a> {
    fn from_boolean_flag(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::BooleanFlag(BooleanFlag::from_element(el)?))
    }

    fn from_category(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Category(Category::from_element(el)?))
    }

    fn from_content_type(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::ContentType(ContentType::from_element(el)?))
    }

    fn from_data_amount(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::DataAmount(DataAmount::from_element(el)?))
    }

    fn from_description(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Description(Description::from_element(el)?))
    }

    fn from_enabled(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Enabled(Enabled::from_element(el)?))
    }

    fn from_experimental(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Experimental(Experimental::from_element(el)?))
    }

    fn from_frequency(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Frequency(Frequency::from_element(el)?))
    }

    fn from_label(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Label(Label::from_element(el)?))
    }

    fn from_memory_address(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::MemoryAddress(MemoryAddress::from_element(el)?))
    }

    fn from_metadata_definition(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::MetadataDefinition(MetadataDefinition::from_element(
            el,
        )?))
    }

    fn from_name(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Name(Name::from_element(el)?))
    }

    fn from_percentage(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Percentage(Percentage::from_element(el)?))
    }

    fn from_period(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Period(Period::from_element(el)?))
    }

    fn from_relational(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Relational(Relational::from_element(el)?))
    }

    fn from_registered(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Registered(Registered::from_element(el)?))
    }

    fn from_setting_definition(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::SettingDefinition(SettingDefinition::from_element(
            el,
        )?))
    }

    fn from_stack_trace(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::StackTrace(StackTrace::from_element(el)?))
    }

    fn from_threshold(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Threshold(Threshold::from_element(el)?))
    }

    fn from_timespan(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Timespan(Timespan::from_element(el)?))
    }

    fn from_timestamp(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Timestamp(Timestamp::from_element(el)?))
    }

    fn from_transition_from(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::TransitionFrom(TransitionFrom::from_element(el)?))
    }

    fn from_transition_to(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::TransitionTo(TransitionTo::from_element(el)?))
    }

    fn from_unsigned(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Unsigned(Unsigned::from_element(el)?))
    }

    fn from_cutoff(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Cutoff(Cutoff::from_element(el)?))
    }

    fn from_mirror(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Mirror(Mirror::from_element(el)?))
    }

    fn from_throttle(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Throttle(Throttle::from_element(el)?))
    }

    fn from_unknown(el: &AnnotationElement<'a>) -> Result<Self> {
        Ok(Self::Unknown(el.values.clone()))
    }

    /// Resolve the parser function for a given class name.
    ///
    /// This can be called to compute a lookup table of class ID to parser
    /// function without having to perform a string class name lookup/compare
    /// on every annotation resolve.
    pub fn resolve_parser(
        class_name: &str,
    ) -> (
        bool,
        fn(&AnnotationElement<'a>) -> Result<AnnotationValue<'a>>,
    ) {
        match class_name {
            "jdk.jfr.BooleanFlag" => (true, Self::from_boolean_flag),
            "jdk.jfr.Category" => (true, Self::from_category),
            "jdk.jfr.ContentType" => (true, Self::from_content_type),
            "jdk.jfr.DataAmount" => (true, Self::from_data_amount),
            "jdk.jfr.Description" => (true, Self::from_description),
            "jdk.jfr.Enabled" => (true, Self::from_enabled),
            "jdk.jfr.Experimental" => (true, Self::from_experimental),
            "jdk.jfr.Frequency" => (true, Self::from_frequency),
            "jdk.jfr.Label" => (true, Self::from_label),
            "jdk.jfr.MemoryAddress" => (true, Self::from_memory_address),
            "jdk.jfr.MetadataDefinition" => (true, Self::from_metadata_definition),
            "jdk.jfr.Name" => (true, Self::from_name),
            "jdk.jfr.Percentage" => (true, Self::from_percentage),
            "jdk.jfr.Period" => (true, Self::from_period),
            "jdk.jfr.Registered" => (true, Self::from_registered),
            "jdk.jfr.Relational" => (true, Self::from_relational),
            "jdk.jfr.SettingDefinition" => (true, Self::from_setting_definition),
            "jdk.jfr.StackTrace" => (true, Self::from_stack_trace),
            "jdk.jfr.Threshold" => (true, Self::from_threshold),
            "jdk.jfr.Timespan" => (true, Self::from_timespan),
            "jdk.jfr.Timestamp" => (true, Self::from_timestamp),
            "jdk.jfr.TransitionFrom" => (true, Self::from_transition_from),
            "jdk.jfr.TransitionTo" => (true, Self::from_transition_to),
            "jdk.jfr.Unsigned" => (true, Self::from_unsigned),
            "jdk.jfr.internal.Cutoff" => (true, Self::from_cutoff),
            "jdk.jfr.internal.Mirror" => (true, Self::from_mirror),
            "jdk.jfr.internal.Throttle" => (true, Self::from_throttle),
            _ => (false, Self::from_unknown),
        }
    }

    /// Construct a parsed annotation from its raw metadata [AnnotationElement] and
    /// associated [ClassElement].
    pub fn from_elements(
        annotation: &AnnotationElement<'a>,
        class: &ClassElement<'a>,
    ) -> Result<Self> {
        if annotation.type_id != class.id {
            return Err(Error::AnnotationParse(format!(
                "class id mismatch: {} != {}",
                annotation.type_id, class.id
            )));
        }

        let parser = Self::resolve_parser(class.name.as_ref()).1;

        parser(annotation)
    }

    /// Whether this annotation type is known to this crate.
    pub fn is_builtin(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }
}

/// A parsed annotation.
pub struct ParsedAnnotation<'class, 'a: 'class> {
    /// The class from which this annotation came.
    pub class: &'class ClassElement<'a>,

    /// The parsed value for this annotation.
    pub value: AnnotationValue<'a>,
}

impl<'class, 'a: 'class> ParsedAnnotation<'class, 'a> {
    /// Construct an instance from an [AnnotationElement] and a [ClassElement].
    ///
    /// The [ClassElement] should match the class ID in the [AnnotationElement].
    ///
    /// Just a convenience function.
    pub fn from_elements(
        annotation: &AnnotationElement<'a>,
        class: &'class ClassElement<'a>,
    ) -> Result<Self> {
        let value = AnnotationValue::from_elements(annotation, class)?;

        Ok(Self { class, value })
    }

    /// Whether this is a known annotation defined in this crate.
    pub fn is_builtin(&self) -> bool {
        self.value.is_builtin()
    }
}
