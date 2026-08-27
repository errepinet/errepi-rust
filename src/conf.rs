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

//! Client connection configurations.

/// Connection configuration for the cron client.
///
/// Mirrors `CronClientConfiguration` of `errepi-py`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CronClientConfiguration {
    /// Host of the cron microservice.
    pub host: String,
    /// Port of the cron microservice.
    pub port: u16,
    /// Number of retry attempts on transient gRPC failures.
    pub max_retries: u32,
    /// Base delay in seconds between retry attempts.
    pub retry_delay_secs: u64,
}

impl Default for CronClientConfiguration {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 50051,
            max_retries: 3,
            retry_delay_secs: 1,
        }
    }
}

/// Connection configuration for the generic registries client.
///
/// Mirrors `RegsClientConfiguration` of `errepi-py`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegsClientConfiguration {
    /// Host of the generic registries microservice.
    pub host: String,
    /// Port of the generic registries microservice.
    pub port: u16,
    /// Number of retry attempts on transient gRPC failures.
    pub max_retries: u32,
    /// Base delay in seconds between retry attempts.
    pub retry_delay_secs: u64,
}

impl Default for RegsClientConfiguration {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 50051,
            max_retries: 3,
            retry_delay_secs: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cron_defaults() {
        let config = CronClientConfiguration::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 50051);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_secs, 1);
    }

    #[test]
    fn regs_defaults() {
        let config = RegsClientConfiguration::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 50051);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay_secs, 1);
    }
}
