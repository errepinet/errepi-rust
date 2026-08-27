/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Rust bindings for Errepi Net microservices.
//!
//! The library mirrors the client surface of `errepi-py`: [`cron::CronConfigurator`]
//! for the CronBridgeService (`protos/cron_bridge.proto`) and
//! [`regs::GenericRegsClient`] for the GenericRegsService
//! (`protos/generic_regs.proto`).

pub mod conf;
pub mod cron;
pub mod error;
pub mod models;
pub mod pb;
pub mod regs;
pub mod retry;

pub use conf::{CronClientConfiguration, RegsClientConfiguration};
pub use models::AppInfo;
