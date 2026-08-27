/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Shared domain models.
//!
//! `AppInfo` is defined in both service protos (`cron_bridge.proto`,
//! `generic_regs.proto`).

use serde::{Deserialize, Serialize};

/// Application information, version, and build details.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppInfo {
    /// Application name.
    pub name: String,
    /// Application version.
    pub version: String,
    /// Build timestamp as a string.
    pub build_timestamp: String,
    /// Build date as a string.
    pub build_date: String,
    /// Build time as a string.
    pub build_time: String,
    /// Build date and time as a string.
    pub build_datetime: String,
    /// Git commit hash.
    pub git_hash: String,
    /// Git branch name.
    pub git_branch: String,
}
