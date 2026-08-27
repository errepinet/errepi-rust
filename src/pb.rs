/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Prost-generated messages and clients for the service protos.
//!
//! The protos are staged at build time with distinct packages
//! (`errepi_cron` / `errepi_regs`) because both declare `package protos` and
//! each defines an `AppInfo` message.

pub mod errepi_cron {
    tonic::include_proto!("errepi_cron");
}

pub mod errepi_regs {
    tonic::include_proto!("errepi_regs");
}
