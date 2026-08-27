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

//! Conversions between cron domain models and the prost-generated messages of
//! `protos/cron_bridge.proto` (AppInfo included).
//!
//! Mirrors `errepi-py` `errepi/cron/conversions.py`.

use std::time::SystemTime;

use chrono::{DateTime, Utc};
use prost_types::Timestamp;

use crate::error::ConversionError;
use crate::models::AppInfo;
use crate::pb::errepi_cron as pb;

use super::models::{
    CronConfiguration, HTTPJob, Job, JobBodyType, JobCreateUpdate, JobExecutionResult,
    JobFrequency, JobHttpMethod, JobStatus, JobType, Ref, RefCreateUpdate,
};

impl TryFrom<&pb::AppInfo> for AppInfo {
    type Error = ConversionError;

    fn try_from(info: &pb::AppInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            name: info.name.clone(),
            version: info.version.clone(),
            build_timestamp: info.build_timestamp.clone(),
            build_date: info.build_date.clone(),
            build_time: info.build_time.clone(),
            build_datetime: info.build_datetime.clone(),
            git_hash: info.git_hash.clone(),
            git_branch: info.git_branch.clone(),
        })
    }
}

impl From<&AppInfo> for pb::AppInfo {
    fn from(info: &AppInfo) -> Self {
        Self {
            name: info.name.clone(),
            version: info.version.clone(),
            build_timestamp: info.build_timestamp.clone(),
            build_date: info.build_date.clone(),
            build_time: info.build_time.clone(),
            build_datetime: info.build_datetime.clone(),
            git_hash: info.git_hash.clone(),
            git_branch: info.git_branch.clone(),
        }
    }
}

impl TryFrom<&pb::CronConfigurationEntry> for CronConfiguration {
    type Error = ConversionError;

    fn try_from(entry: &pb::CronConfigurationEntry) -> Result<Self, Self::Error> {
        Ok(Self {
            job_max_retries: entry.job_max_retries,
            job_retry_delay_secs: entry.job_retry_delay_secs,
            set_at: match &entry.set_at {
                Some(ts) => Some(to_datetime(ts)?),
                None => None,
            },
        })
    }
}

impl From<&CronConfiguration> for pb::CronConfigurationEntry {
    fn from(config: &CronConfiguration) -> Self {
        Self {
            job_max_retries: config.job_max_retries,
            job_retry_delay_secs: config.job_retry_delay_secs,
            set_at: config.set_at.as_ref().map(to_timestamp),
        }
    }
}

impl From<&CronConfiguration> for pb::CronConfigurationEntrySet {
    fn from(config: &CronConfiguration) -> Self {
        Self {
            job_max_retries: config.job_max_retries,
            job_retry_delay_secs: config.job_retry_delay_secs,
        }
    }
}

impl TryFrom<&pb::CronRef> for Ref {
    type Error = ConversionError;

    fn try_from(r: &pb::CronRef) -> Result<Self, Self::Error> {
        Ok(Self {
            setted_at: to_datetime(
                r.setted_at
                    .as_ref()
                    .ok_or(ConversionError::MissingField("setted_at"))?,
            )?,
            value: r.value.clone(),
        })
    }
}

impl From<&Ref> for pb::CronRef {
    fn from(r: &Ref) -> Self {
        Self {
            setted_at: Some(to_timestamp(&r.setted_at)),
            value: r.value.clone(),
        }
    }
}

impl From<&RefCreateUpdate> for pb::CronRefCreateUpdate {
    fn from(r: &RefCreateUpdate) -> Self {
        Self {
            value: r.value.clone(),
        }
    }
}

impl From<&JobFrequency> for pb::CronJobFrequency {
    fn from(frequency: &JobFrequency) -> Self {
        use pb::cron_job_frequency::Frequency;
        let frequency = Some(match frequency {
            JobFrequency::Hour(value) => Frequency::Hour(*value),
            JobFrequency::Day(value) => Frequency::Day(*value),
            JobFrequency::Week(value) => Frequency::Week(*value),
            JobFrequency::Month(value) => Frequency::Month(*value),
            JobFrequency::Minute(value) => Frequency::Minute(*value),
        });
        Self { frequency }
    }
}

impl TryFrom<&pb::CronJobFrequency> for JobFrequency {
    type Error = ConversionError;

    fn try_from(frequency: &pb::CronJobFrequency) -> Result<Self, Self::Error> {
        use pb::cron_job_frequency::Frequency;
        match &frequency.frequency {
            Some(Frequency::Hour(value)) => Ok(Self::Hour(*value)),
            Some(Frequency::Day(value)) => Ok(Self::Day(*value)),
            Some(Frequency::Week(value)) => Ok(Self::Week(*value)),
            Some(Frequency::Month(value)) => Ok(Self::Month(*value)),
            Some(Frequency::Minute(value)) => Ok(Self::Minute(*value)),
            None => Err(ConversionError::InvalidOneof(
                "frequency",
                None,
            )),
        }
    }
}

impl TryFrom<i32> for JobHttpMethod {
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            v if v == pb::CronJobHttpMethod::Get as i32 => Ok(Self::Get),
            v if v == pb::CronJobHttpMethod::Post as i32 => Ok(Self::Post),
            v if v == pb::CronJobHttpMethod::Patch as i32 => Ok(Self::Patch),
            v if v == pb::CronJobHttpMethod::Put as i32 => Ok(Self::Put),
            v if v == pb::CronJobHttpMethod::Delete as i32 => Ok(Self::Delete),
            other => Err(ConversionError::InvalidEnum("CronJobHttpMethod", other)),
        }
    }
}

impl From<&JobHttpMethod> for i32 {
    fn from(method: &JobHttpMethod) -> Self {
        match method {
            JobHttpMethod::Get => pb::CronJobHttpMethod::Get as i32,
            JobHttpMethod::Post => pb::CronJobHttpMethod::Post as i32,
            JobHttpMethod::Patch => pb::CronJobHttpMethod::Patch as i32,
            JobHttpMethod::Put => pb::CronJobHttpMethod::Put as i32,
            JobHttpMethod::Delete => pb::CronJobHttpMethod::Delete as i32,
        }
    }
}

impl TryFrom<i32> for JobBodyType {
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            v if v == pb::CronJobBodyType::Json as i32 => Ok(Self::Json),
            v if v == pb::CronJobBodyType::Text as i32 => Ok(Self::Text),
            other => Err(ConversionError::InvalidEnum("CronJobBodyType", other)),
        }
    }
}

impl From<&JobBodyType> for i32 {
    fn from(body_type: &JobBodyType) -> Self {
        match body_type {
            JobBodyType::Json => pb::CronJobBodyType::Json as i32,
            JobBodyType::Text => pb::CronJobBodyType::Text as i32,
        }
    }
}

impl TryFrom<i32> for JobStatus {
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            v if v == pb::CronJobStatus::Scheduled as i32 => Ok(Self::Scheduled),
            v if v == pb::CronJobStatus::Rescheduled as i32 => Ok(Self::Rescheduled),
            v if v == pb::CronJobStatus::RetryScheduled as i32 => Ok(Self::RetryScheduled),
            v if v == pb::CronJobStatus::Ok as i32 => Ok(Self::Ok),
            v if v == pb::CronJobStatus::Failed as i32 => Ok(Self::Failed),
            other => Err(ConversionError::InvalidEnum("CronJobStatus", other)),
        }
    }
}

impl From<&JobStatus> for i32 {
    fn from(status: &JobStatus) -> Self {
        match status {
            JobStatus::Scheduled => pb::CronJobStatus::Scheduled as i32,
            JobStatus::Rescheduled => pb::CronJobStatus::Rescheduled as i32,
            JobStatus::RetryScheduled => pb::CronJobStatus::RetryScheduled as i32,
            JobStatus::Ok => pb::CronJobStatus::Ok as i32,
            JobStatus::Failed => pb::CronJobStatus::Failed as i32,
        }
    }
}

impl From<&HTTPJob> for pb::CronHttpJob {
    fn from(job: &HTTPJob) -> Self {
        Self {
            url: job.url.clone(),
            method: i32::from(&job.method),
            headers: job.headers.clone().unwrap_or_default(),
            body: job.body.clone(),
            body_type: job.body_type.as_ref().map(i32::from),
            timeout_seconds: job.timeout_seconds,
            user_agent: job.user_agent.clone(),
            valid_http_codes: job.valid_http_codes.clone().unwrap_or_default(),
        }
    }
}

impl TryFrom<&pb::CronHttpJob> for HTTPJob {
    type Error = ConversionError;

    fn try_from(job: &pb::CronHttpJob) -> Result<Self, Self::Error> {
        let headers = if job.headers.is_empty() {
            None
        } else {
            Some(job.headers.clone())
        };
        let valid_http_codes = if job.valid_http_codes.is_empty() {
            None
        } else {
            Some(job.valid_http_codes.clone())
        };
        Ok(Self {
            url: job.url.clone(),
            method: JobHttpMethod::try_from(job.method)?,
            headers,
            body: job.body.clone(),
            body_type: match job.body_type {
                Some(value) => Some(JobBodyType::try_from(value)?),
                None => None,
            },
            timeout_seconds: job.timeout_seconds,
            user_agent: job.user_agent.clone(),
            valid_http_codes,
        })
    }
}

impl From<&JobType> for pb::CronJobType {
    fn from(job_type: &JobType) -> Self {
        use pb::cron_job_type::JobType as PbJobType;
        let job_type = Some(match job_type {
            JobType::Http(job) => PbJobType::Http(job.into()),
        });
        Self { job_type }
    }
}

impl TryFrom<&pb::CronJobType> for JobType {
    type Error = ConversionError;

    fn try_from(job_type: &pb::CronJobType) -> Result<Self, Self::Error> {
        use pb::cron_job_type::JobType as PbJobType;
        match &job_type.job_type {
            Some(PbJobType::Http(job)) => Ok(Self::Http(job.try_into()?)),
            None => Err(ConversionError::InvalidOneof("job_type", None)),
        }
    }
}

impl TryFrom<&pb::CronJob> for Job {
    type Error = ConversionError;

    fn try_from(job: &pb::CronJob) -> Result<Self, Self::Error> {
        Ok(Self {
            id: job.id.clone(),
            description: job.description.clone(),
            frequency: job
                .frequency
                .as_ref()
                .map(JobFrequency::try_from)
                .transpose()?,
            created: to_datetime(
                job.created
                    .as_ref()
                    .ok_or(ConversionError::MissingField("created"))?,
            )?,
            updated: job
                .updated
                .as_ref()
                .map(to_datetime)
                .transpose()?,
            last_execution_dt: job
                .last_execution_dt
                .as_ref()
                .map(to_datetime)
                .transpose()?,
            next_execution_dt: to_datetime(
                job.next_execution_dt
                    .as_ref()
                    .ok_or(ConversionError::MissingField("next_execution_dt"))?,
            )?,
            enabled: job.enabled,
            job_type: job
                .job_type
                .as_ref()
                .ok_or(ConversionError::MissingField("job_type"))?
                .try_into()?,
            job_status: JobStatus::try_from(job.job_status)?,
            configuration: job
                .configuration
                .as_ref()
                .ok_or(ConversionError::MissingField("configuration"))?
                .try_into()?,
            curr_retries: job.curr_retries,
        })
    }
}

impl TryFrom<&Job> for pb::CronJob {
    type Error = ConversionError;

    fn try_from(job: &Job) -> Result<Self, Self::Error> {
        Ok(Self {
            id: job.id.clone(),
            description: job.description.clone(),
            frequency: job.frequency.as_ref().map(pb::CronJobFrequency::from),
            created: Some(to_timestamp(&job.created)),
            updated: job.updated.as_ref().map(to_timestamp),
            last_execution_dt: job.last_execution_dt.as_ref().map(to_timestamp),
            next_execution_dt: Some(to_timestamp(&job.next_execution_dt)),
            enabled: job.enabled,
            job_type: Some((&job.job_type).into()),
            job_status: i32::from(&job.job_status),
            configuration: Some((&job.configuration).into()),
            curr_retries: job.curr_retries,
        })
    }
}

impl From<&JobCreateUpdate> for pb::CronJobCreateUpdate {
    fn from(job: &JobCreateUpdate) -> Self {
        Self {
            description: job.description.clone(),
            frequency: job.frequency.as_ref().map(pb::CronJobFrequency::from),
            enabled: job.enabled,
            job_type: Some((&job.job_type).into()),
            next_execution_dt: Some(to_timestamp(&job.next_execution_dt)),
            use_configuration: job.use_configuration.clone(),
        }
    }
}

impl TryFrom<&pb::CronJobExecutionResult> for JobExecutionResult {
    type Error = ConversionError;

    fn try_from(result: &pb::CronJobExecutionResult) -> Result<Self, Self::Error> {
        Ok(Self {
            date_time: to_datetime(
                result
                    .date_time
                    .as_ref()
                    .ok_or(ConversionError::MissingField("date_time"))?,
            )?,
            detail: result.detail.clone(),
            is_success: result.is_success,
            job_id: result.job_id.clone(),
            namespace: result.namespace.clone(),
        })
    }
}

/// Convert a protobuf timestamp to a UTC datetime.
pub(crate) fn to_datetime(ts: &Timestamp) -> Result<DateTime<Utc>, ConversionError> {
    Ok(SystemTime::try_from(*ts)?.into())
}

/// Convert a UTC datetime to a protobuf timestamp.
pub(crate) fn to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
    Timestamp::from(SystemTime::from(*dt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::pb::errepi_cron::{
        self, cron_job_frequency::Frequency, cron_job_type::JobType as PbJobType,
    };
    use chrono::TimeZone;

    fn ts(seconds: i64) -> Timestamp {
        Timestamp {
            seconds,
            nanos: 0,
        }
    }

    fn dt(seconds: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(seconds, 0).unwrap()
    }

    #[test]
    fn app_info_roundtrip() {
        let info = AppInfo {
            name: "cron".into(),
            version: "1.2.3".into(),
            build_timestamp: "123".into(),
            build_date: "2024-01-01".into(),
            build_time: "10:00:00 UTC".into(),
            build_datetime: "2024-01-01 10:00:00 UTC".into(),
            git_hash: "abc123".into(),
            git_branch: "main".into(),
        };
        let pb_info = pb::AppInfo::from(&info);
        let back = AppInfo::try_from(&pb_info).unwrap();
        assert_eq!(back, info);
    }

    #[test]
    fn configuration_roundtrip() {
        let config = CronConfiguration {
            job_max_retries: 3,
            job_retry_delay_secs: 60,
            set_at: Some(dt(1_700_000_000)),
        };
        let pb_config = pb::CronConfigurationEntry::from(&config);
        let back = CronConfiguration::try_from(&pb_config).unwrap();
        assert_eq!(back, config);
    }

    #[test]
    fn configuration_without_set_at_roundtrip() {
        let config = CronConfiguration {
            job_max_retries: 3,
            job_retry_delay_secs: 60,
            set_at: None,
        };
        let pb_config = pb::CronConfigurationEntry::from(&config);
        assert!(pb_config.set_at.is_none());
        let back = CronConfiguration::try_from(&pb_config).unwrap();
        assert_eq!(back, config);
    }

    #[test]
    fn configuration_set_roundtrip() {
        let config = CronConfiguration {
            job_max_retries: 5,
            job_retry_delay_secs: 10,
            set_at: None,
        };
        let pb_config = pb::CronConfigurationEntrySet::from(&config);
        assert_eq!(pb_config.job_max_retries, 5);
        assert_eq!(pb_config.job_retry_delay_secs, 10);
    }

    #[test]
    fn ref_roundtrip() {
        let r = Ref {
            setted_at: dt(1_700_000_000),
            value: "my-value".into(),
        };
        let pb_ref = pb::CronRef::from(&r);
        let back = Ref::try_from(&pb_ref).unwrap();
        assert_eq!(back, r);
    }

    #[test]
    fn ref_create_update_roundtrip() {
        let r = RefCreateUpdate {
            value: "my-value".into(),
        };
        let pb_ref = pb::CronRefCreateUpdate::from(&r);
        assert_eq!(pb_ref.value, "my-value");
    }

    #[test]
    fn frequency_maps_every_variant() {
        let variants = [
            JobFrequency::Hour(1),
            JobFrequency::Day(2),
            JobFrequency::Week(3),
            JobFrequency::Month(4),
            JobFrequency::Minute(5),
        ];
        for frequency in variants {
            let pb_freq = pb::CronJobFrequency::from(&frequency);
            assert_eq!(JobFrequency::try_from(&pb_freq).unwrap(), frequency);
        }
    }

    #[test]
    fn frequency_none_fails() {
        let pb_freq = pb::CronJobFrequency { frequency: None };
        let err = JobFrequency::try_from(&pb_freq).unwrap_err();
        assert!(matches!(err, ConversionError::InvalidOneof("frequency", None)));
    }

    #[test]
    fn frequency_pb_variant_mapping() {
        let pb_freq = pb::CronJobFrequency {
            frequency: Some(Frequency::Minute(15)),
        };
        assert_eq!(
            JobFrequency::try_from(&pb_freq).unwrap(),
            JobFrequency::Minute(15)
        );
    }

    #[test]
    fn http_method_roundtrip() {
        let methods = [
            JobHttpMethod::Get,
            JobHttpMethod::Post,
            JobHttpMethod::Patch,
            JobHttpMethod::Put,
            JobHttpMethod::Delete,
        ];
        for method in methods {
            assert_eq!(JobHttpMethod::try_from(i32::from(&method)).unwrap(), method);
        }
        assert!(matches!(
            JobHttpMethod::try_from(99),
            Err(ConversionError::InvalidEnum("CronJobHttpMethod", 99))
        ));
    }

    #[test]
    fn body_type_roundtrip() {
        for body_type in [JobBodyType::Json, JobBodyType::Text] {
            assert_eq!(
                JobBodyType::try_from(i32::from(&body_type)).unwrap(),
                body_type
            );
        }
    }

    #[test]
    fn job_status_roundtrip() {
        let statuses = [
            JobStatus::Scheduled,
            JobStatus::Rescheduled,
            JobStatus::RetryScheduled,
            JobStatus::Ok,
            JobStatus::Failed,
        ];
        for status in statuses {
            assert_eq!(JobStatus::try_from(i32::from(&status)).unwrap(), status);
        }
        assert!(matches!(
            JobStatus::try_from(99),
            Err(ConversionError::InvalidEnum("CronJobStatus", 99))
        ));
    }

    #[test]
    fn http_job_roundtrip_full() {
        let job = HTTPJob {
            body: Some("{\"key\": \"value\"}".into()),
            body_type: Some(JobBodyType::Json),
            headers: Some(HashMap::from([("Authorization".into(), "Bearer x".into())])),
            method: JobHttpMethod::Post,
            timeout_seconds: Some(30),
            url: "https://example.com/api".into(),
            user_agent: Some("agent".into()),
            valid_http_codes: Some(vec![200, 201]),
        };
        let pb_job = pb::CronHttpJob::from(&job);
        let back = HTTPJob::try_from(&pb_job).unwrap();
        assert_eq!(back, job);
    }

    #[test]
    fn http_job_roundtrip_minimal() {
        let job = HTTPJob {
            body: None,
            body_type: None,
            headers: None,
            method: JobHttpMethod::Get,
            timeout_seconds: None,
            url: "https://example.com".into(),
            user_agent: None,
            valid_http_codes: None,
        };
        let pb_job = pb::CronHttpJob::from(&job);
        let back = HTTPJob::try_from(&pb_job).unwrap();
        assert_eq!(back, job);
        assert!(pb_job.headers.is_empty());
        assert!(pb_job.valid_http_codes.is_empty());
    }

    #[test]
    fn job_type_roundtrip() {
        let job_type = JobType::Http(HTTPJob {
            body: None,
            body_type: None,
            headers: None,
            method: JobHttpMethod::Put,
            timeout_seconds: None,
            url: "https://example.com".into(),
            user_agent: None,
            valid_http_codes: None,
        });
        let pb_job_type = pb::CronJobType::from(&job_type);
        assert!(matches!(
            pb_job_type.job_type,
            Some(PbJobType::Http(_))
        ));
        let back = JobType::try_from(&pb_job_type).unwrap();
        assert_eq!(back, job_type);
    }

    #[test]
    fn job_type_none_fails() {
        let pb_job_type = pb::CronJobType { job_type: None };
        let err = JobType::try_from(&pb_job_type).unwrap_err();
        assert!(matches!(err, ConversionError::InvalidOneof("job_type", None)));
    }

    #[test]
    fn job_roundtrip_full() {
        let job = Job {
            id: Some("job-1".into()),
            description: Some("desc".into()),
            frequency: Some(JobFrequency::Minute(5)),
            created: dt(1_700_000_000),
            updated: Some(dt(1_700_000_100)),
            last_execution_dt: Some(dt(1_700_000_050)),
            next_execution_dt: dt(1_700_003_600),
            enabled: true,
            job_type: JobType::Http(HTTPJob {
                body: None,
                body_type: None,
                headers: None,
                method: JobHttpMethod::Post,
                timeout_seconds: None,
                url: "https://example.com".into(),
                user_agent: None,
                valid_http_codes: None,
            }),
            job_status: JobStatus::RetryScheduled,
            configuration: CronConfiguration {
                job_max_retries: 3,
                job_retry_delay_secs: 60,
                set_at: Some(dt(1_700_000_000)),
            },
            curr_retries: 2,
        };
        let pb_job = pb::CronJob::try_from(&job).unwrap();
        let back = Job::try_from(&pb_job).unwrap();
        assert_eq!(back, job);
    }

    #[test]
    fn job_roundtrip_minimal() {
        let job = Job {
            id: None,
            description: None,
            frequency: None,
            created: dt(1_700_000_000),
            updated: None,
            last_execution_dt: None,
            next_execution_dt: dt(1_700_003_600),
            enabled: false,
            job_type: JobType::Http(HTTPJob {
                body: None,
                body_type: None,
                headers: None,
                method: JobHttpMethod::Get,
                timeout_seconds: None,
                url: "https://example.com".into(),
                user_agent: None,
                valid_http_codes: None,
            }),
            job_status: JobStatus::Scheduled,
            configuration: CronConfiguration {
                job_max_retries: 0,
                job_retry_delay_secs: 0,
                set_at: None,
            },
            curr_retries: 0,
        };
        let pb_job = pb::CronJob::try_from(&job).unwrap();
        let back = Job::try_from(&pb_job).unwrap();
        assert_eq!(back, job);
    }

    #[test]
    fn job_missing_created_fails() {
        let pb_job = pb::CronJob {
            id: None,
            description: None,
            frequency: None,
            created: None,
            updated: None,
            last_execution_dt: None,
            next_execution_dt: None,
            enabled: false,
            job_type: None,
            job_status: 0,
            configuration: None,
            curr_retries: 0,
        };
        let err = Job::try_from(&pb_job).unwrap_err();
        assert!(matches!(err, ConversionError::MissingField("created")));
    }

    #[test]
    fn job_create_update_roundtrip() {
        let job = JobCreateUpdate {
            description: Some("Test job".into()),
            enabled: true,
            frequency: Some(JobFrequency::Minute(5)),
            job_type: JobType::Http(HTTPJob {
                body: None,
                body_type: None,
                headers: None,
                method: JobHttpMethod::Post,
                timeout_seconds: None,
                url: "https://example.com".into(),
                user_agent: None,
                valid_http_codes: None,
            }),
            next_execution_dt: dt(1_700_003_600),
            use_configuration: Some("main".into()),
        };
        let pb_job = pb::CronJobCreateUpdate::from(&job);
        assert_eq!(pb_job.description.as_deref(), Some("Test job"));
        assert_eq!(pb_job.use_configuration.as_deref(), Some("main"));
        assert_eq!(
            JobFrequency::try_from(&pb_job.frequency.unwrap()).unwrap(),
            JobFrequency::Minute(5)
        );
        assert_eq!(
            JobType::try_from(&pb_job.job_type.unwrap()).unwrap(),
            job.job_type
        );
    }

    #[test]
    fn job_execution_result_roundtrip() {
        let result = JobExecutionResult {
            date_time: dt(1_700_000_000),
            detail: "ok".into(),
            is_success: true,
            job_id: "job-1".into(),
            namespace: "default".into(),
        };
        let pb_result = errepi_cron::CronJobExecutionResult {
            job_id: result.job_id.clone(),
            date_time: Some(ts(1_700_000_000)),
            detail: result.detail.clone(),
            is_success: result.is_success,
            namespace: result.namespace.clone(),
        };
        let back = JobExecutionResult::try_from(&pb_result).unwrap();
        assert_eq!(back, result);
    }
}
