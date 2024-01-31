// Copyright 2023 Gregory Szorc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JFR event settings.
//!
//! This module holds types representing the event settings captured in
//! chunk metadata. These represent types in the jdk.jfr.settings package.

use crate::{
    error::{Error, Result},
    metadata::{ClassElement, SettingsElement},
};
use std::{borrow::Cow, ops::Deref, str::FromStr};

/// Represents a unit of time with infinity as a possibility.
#[derive(Clone, Copy, Debug)]
pub enum TimespanWithInfinity {
    Infinity,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

impl FromStr for TimespanWithInfinity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "infinity" {
            Ok(Self::Infinity)
        } else {
            let (number, unit) = s
                .split_once(' ')
                .ok_or_else(|| Error::SettingParse(format!("timespan has invalid format: {s}")))?;

            let number = u64::from_str(number).map_err(|e| {
                Error::SettingParse(format!("failed to parse timespan integer: {e}"))
            })?;

            match unit {
                "ns" => Ok(Self::Nanoseconds(number)),
                "us" => Ok(Self::Microseconds(number)),
                "ms" => Ok(Self::Milliseconds(number)),
                "s" => Ok(Self::Seconds(number)),
                "m" => Ok(Self::Minutes(number)),
                "h" => Ok(Self::Hours(number)),
                "d" => Ok(Self::Days(number)),
                _ => Err(Error::SettingParse(format!(
                    "timespan has invalid unit: {unit}"
                ))),
            }
        }
    }
}

/// Limits running time of an event.
#[derive(Clone, Debug)]
pub struct Cutoff(TimespanWithInfinity);

impl Deref for Cutoff {
    type Target = TimespanWithInfinity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Cutoff {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(TimespanWithInfinity::from_str(s)?))
    }
}

/// Whether something is enabled.
#[derive(Clone, Debug)]
pub struct Enabled(bool);

impl Deref for Enabled {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Enabled {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self(true)),
            "false" => Ok(Self(false)),
            _ => Err(Error::SettingParse(format!(
                "jdk.jfr.setting.Enabled illegal value: {s}"
            ))),
        }
    }
}

/// How often the event should be emitted.
#[derive(Clone, Debug)]
pub enum Period {
    EveryChunk,
    BeginChunk,
    EndChunk,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

impl FromStr for Period {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "everyChunk" => Ok(Self::EveryChunk),
            "beginChunk" => Ok(Self::BeginChunk),
            "endChunk" => Ok(Self::EndChunk),
            _ => {
                let (number, unit) = s.split_once(' ').ok_or_else(|| {
                    Error::SettingParse(format!("jdk.jfr.setting.Period illegal format: {s}"))
                })?;

                let number = u64::from_str(number).map_err(|e| {
                    Error::AnnotationParse(format!(
                        "jdk.jfr.setting.Period failed to parse integer: {e}"
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
                        "jdk.jfr.setting.Period has invalid unit: {unit}"
                    ))),
                }
            }
        }
    }
}

/// Whether to emit stack traces for events.
#[derive(Clone, Debug)]
pub struct StackTrace(bool);

impl Deref for StackTrace {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for StackTrace {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self(true)),
            "false" => Ok(Self(false)),
            _ => Err(Error::SettingParse(format!(
                "jdk.jfr.setting.StackTrace illegal value: {s}"
            ))),
        }
    }
}

/// Record events with duration above a time threshold.
#[derive(Clone, Debug)]
pub struct Threshold(TimespanWithInfinity);

impl Deref for Threshold {
    type Target = TimespanWithInfinity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Threshold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(TimespanWithInfinity::from_str(s)?))
    }
}

/// Throttle the emission rate of an event.
#[derive(Clone, Debug)]
pub enum Throttle {
    Off,
    Nanoseconds(u64),
    Microseconds(u64),
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

impl FromStr for Throttle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "off" {
            Ok(Self::Off)
        } else {
            let (number, unit) = s.split_once('/').ok_or_else(|| {
                Error::SettingParse(format!("jdk.jfr.setting.Throttle has invalid format: {s}"))
            })?;

            let number = u64::from_str(number).map_err(|e| {
                Error::AnnotationParse(format!(
                    "jdk.jfr.setting.Throttle failed to parse integer: {e}"
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
                    "jdk.jfr.setting.Throttle has invalid unit: {unit}"
                ))),
            }
        }
    }
}

/// A generic setting that we don't know anything more about.
#[derive(Clone, Debug)]
pub struct GenericSetting<'class, 'data: 'class> {
    pub class: &'class ClassElement<'data>,
    pub default_value: Cow<'data, str>,
}

/// A parsed setting value.
#[derive(Clone, Debug)]
pub enum SettingValue<'chunk> {
    Cutoff(Cutoff),
    Enabled(Enabled),
    Period(Period),
    StackTrace(StackTrace),
    Threshold(Threshold),
    Throttle(Throttle),
    Unknown(Cow<'chunk, str>),
}

impl<'chunk> SettingValue<'chunk> {
    pub fn from_cutoff(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Cutoff(Cutoff::from_str(el.default_value.as_ref())?))
    }

    pub fn from_enabled(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Enabled(Enabled::from_str(el.default_value.as_ref())?))
    }

    pub fn from_period(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Period(Period::from_str(el.default_value.as_ref())?))
    }

    pub fn from_stack_trace(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::StackTrace(StackTrace::from_str(
            el.default_value.as_ref(),
        )?))
    }

    pub fn from_threshold(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Threshold(Threshold::from_str(
            el.default_value.as_ref(),
        )?))
    }

    pub fn from_throttle(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Throttle(Throttle::from_str(
            el.default_value.as_ref(),
        )?))
    }

    pub fn from_unknown(el: &SettingsElement<'chunk>) -> Result<Self> {
        Ok(Self::Unknown(el.default_value.clone()))
    }

    /// Resolve a parser for a setting name.
    ///
    /// Boolean indicates whether this is a setting type known to this crate.
    pub fn resolve_parser(
        name: &str,
    ) -> (
        bool,
        fn(&SettingsElement<'chunk>) -> Result<SettingValue<'chunk>>,
    ) {
        match name {
            "cutoff" => (true, Self::from_cutoff),
            "enabled" => (true, Self::from_enabled),
            "period" => (true, Self::from_period),
            "stackTrace" => (true, Self::from_stack_trace),
            "threshold" => (true, Self::from_threshold),
            "throttle" => (true, Self::from_throttle),
            _ => (false, Self::from_unknown),
        }
    }

    /// Construct an instance from a [SettingsElement].
    ///
    /// This will trigger parsing of the setting value for known setting types.
    pub fn from_element(setting: &SettingsElement<'chunk>) -> Result<Self> {
        let parser = Self::resolve_parser(setting.name.as_ref()).1;

        parser(setting)
    }
}

#[derive(Clone, Debug)]
pub struct Setting<'el, 'chunk: 'el> {
    pub element: &'el SettingsElement<'chunk>,
    pub value: SettingValue<'chunk>,
}

impl<'el, 'chunk: 'el> Setting<'el, 'chunk> {
    /// Construct an instance from a [SettingsElement] instance.
    pub fn from_element(element: &'el SettingsElement<'chunk>) -> Result<Self> {
        let value = SettingValue::from_element(element)?;

        Ok(Self { element, value })
    }
}
