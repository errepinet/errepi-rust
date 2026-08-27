/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Retry helper for gRPC calls: transient failures (connection unavailable,
//! deadline exceeded) are retried with exponential backoff before returning
//! the last error.
//!
//! Retries transient gRPC failures with exponential backoff.

use std::time::Duration;

use tonic::Code;

/// Whether the gRPC status code is a transient failure worth retrying.
pub fn is_retryable(code: Code) -> bool {
    matches!(code, Code::Unavailable | Code::DeadlineExceeded)
}

/// Call `f`, retrying on transient gRPC failures with exponential backoff.
///
/// `retries` is the number of retry attempts after the first failure,
/// `delay` is the base delay between attempts (doubled each retry).
/// Non-transient errors are returned immediately; the last error is returned
/// once retries are exhausted.
pub async fn call_with_retry<T, F, Fut>(
    mut f: F,
    retries: u32,
    delay: Duration,
) -> Result<T, tonic::Status>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, tonic::Status>>,
{
    let mut last_error: Option<tonic::Status> = None;
    for attempt in 0..=retries {
        match f().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if !is_retryable(error.code()) {
                    return Err(error);
                }
                last_error = Some(error);
                if attempt < retries {
                    tokio::time::sleep(delay * 2u32.saturating_pow(attempt)).await;
                }
            }
        }
    }
    Err(last_error.expect("at least one attempt ran"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::{Code, Status};

    #[test]
    fn retryable_codes() {
        assert!(is_retryable(Code::Unavailable));
        assert!(is_retryable(Code::DeadlineExceeded));
        assert!(!is_retryable(Code::Ok));
        assert!(!is_retryable(Code::NotFound));
        assert!(!is_retryable(Code::InvalidArgument));
        assert!(!is_retryable(Code::Internal));
    }

    #[tokio::test]
    async fn succeeds_on_first_attempt() {
        let result = call_with_retry(
            || async { Ok::<_, Status>(42) },
            3,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn retries_transient_then_succeeds() {
        let mut calls = 0;
        let result = call_with_retry(
            || {
                calls += 1;
                async move {
                    if calls < 3 {
                        Err(Status::unavailable("not ready"))
                    } else {
                        Ok::<_, Status>("ok")
                    }
                }
            },
            3,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(result.unwrap(), "ok");
        assert_eq!(calls, 3);
    }

    #[tokio::test]
    async fn returns_last_error_when_exhausted() {
        let result = call_with_retry(
            || async { Err::<(), Status>(Status::unavailable("down")) },
            2,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(result.unwrap_err().code(), Code::Unavailable);
    }

    #[tokio::test]
    async fn non_transient_error_returns_immediately() {
        let mut calls = 0;
        let result = call_with_retry(
            || {
                calls += 1;
                async move { Err::<(), Status>(Status::not_found("missing")) }
            },
            3,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(result.unwrap_err().code(), Code::NotFound);
        assert_eq!(calls, 1);
    }
}
