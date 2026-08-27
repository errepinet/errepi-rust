/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * All rights reserved. This software is the property of Errepi Net S.R.L.
 * Unauthorized copying, modification, distribution, or use of this software,
 * via any medium, is strictly prohibited without express written permission.
 */

//! Errors raised when converting between prost-generated messages and domain
//! models.

use std::fmt;

/// Conversion failure between a protobuf message and a domain model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionError {
    /// A required protobuf field was not set.
    MissingField(&'static str),
    /// An enum value is not recognized.
    InvalidEnum(&'static str, i32),
    /// A oneof holds no variant or an unsupported one.
    InvalidOneof(&'static str, Option<String>),
    /// A timestamp is out of the representable range.
    Timestamp(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "missing required field: {field}"),
            Self::InvalidEnum(name, value) => write!(f, "invalid {name} value: {value}"),
            Self::InvalidOneof(name, variant) => {
                write!(f, "unsupported {name} oneof: {:?}", variant)
            }
            Self::Timestamp(detail) => write!(f, "invalid timestamp: {detail}"),
        }
    }
}

impl std::error::Error for ConversionError {}

impl From<prost_types::TimestampError> for ConversionError {
    fn from(err: prost_types::TimestampError) -> Self {
        Self::Timestamp(err.to_string())
    }
}
