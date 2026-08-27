/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Client for the Errepi Net Cron microservice (CronBridgeService).
//!
//! Mirrors `errepi-py` `errepi/cron/__init__.py`.

pub mod conversions;
pub mod models;

use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::conf::CronClientConfiguration;
use crate::error::ConversionError;
use crate::models::AppInfo;
use crate::pb::errepi_cron as pb;
use crate::pb::errepi_cron::cron_bridge_service_client::CronBridgeServiceClient;
use crate::retry;

pub use models::{
    CronConfiguration, HTTPJob, Job, JobBodyType, JobCreateUpdate, JobExecutionResult,
    JobFrequency, JobHttpMethod, JobStatus, JobType, Ref, RefCreateUpdate,
};

fn conversion_error(error: ConversionError) -> tonic::Status {
    tonic::Status::internal(format!("conversion failed: {error}"))
}

/// Create a `JobType` instance encapsulating the given HTTP job.
pub fn http_job_type(http_job: HTTPJob) -> JobType {
    JobType::Http(http_job)
}

/// Client for interacting with the Errepi Net Cron microservice
/// (CronBridgeService).
///
/// Provides methods to retrieve application info, manage job configurations,
/// refs and scheduled jobs over gRPC. The interface mirrors the RPCs of
/// `protos/cron_bridge.proto`: every operation takes the `tenant_id` and
/// `namespace` of the resource.
#[derive(Clone)]
pub struct CronConfigurator {
    stub: CronBridgeServiceClient<Channel>,
    config: CronClientConfiguration,
}

impl CronConfigurator {
    /// Connect to the cron microservice at the configured `host:port`
    /// (defaults to `localhost:50051`).
    pub async fn new(config: CronClientConfiguration) -> Result<Self, tonic::transport::Error> {
        let endpoint = format!("http://{}:{}", config.host, config.port);
        let channel = Endpoint::from_shared(endpoint)?
            .connect()
            .await?;
        Ok(Self {
            stub: CronBridgeServiceClient::new(channel),
            config,
        })
    }

    /// Retrieve application build and version information (GetAppInfo).
    pub async fn app_info(&mut self) -> Result<AppInfo, tonic::Status> {
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                async move { client.get_app_info(pb::Empty {}).await }
            },
            retries,
            delay,
        )
        .await?;
        AppInfo::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Get a job configuration entry by namespace and name
    /// (CronConfigurationGet).
    pub async fn get_configuration(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        name: &str,
    ) -> Result<CronConfiguration, tonic::Status> {
        let request = pb::CronConfigurationGetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            name: name.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_configuration_get(request).await }
            },
            retries,
            delay,
        )
        .await?;
        CronConfiguration::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Set or update a job configuration entry (CronConfigurationSet).
    pub async fn set_configuration(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        name: &str,
        config: &CronConfiguration,
    ) -> Result<CronConfiguration, tonic::Status> {
        let request = pb::CronConfigurationSetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            name: name.into(),
            configuration: Some((config).into()),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_configuration_set(request).await }
            },
            retries,
            delay,
        )
        .await?;
        CronConfiguration::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Remove a job configuration entry (CronConfigurationUnset).
    pub async fn unset_configuration(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        name: &str,
    ) -> Result<(), tonic::Status> {
        let request = pb::CronConfigurationUnsetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            name: name.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_configuration_unset(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Ok(())
    }

    /// List all jobs in a given namespace (CronJobsList).
    pub async fn list_jobs(
        &mut self,
        tenant_id: &str,
        namespace: &str,
    ) -> Result<Vec<Job>, tonic::Status> {
        let request = pb::CronJobsListRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_jobs_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .jobs
            .iter()
            .map(Job::try_from)
            .map(|job| job.map_err(conversion_error))
            .collect()
    }

    /// Create a new scheduled job in the given namespace (CronJobCreate).
    pub async fn create_job(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        job: &JobCreateUpdate,
    ) -> Result<Job, tonic::Status> {
        let request = pb::CronJobCreateRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            job: Some(job.into()),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_job_create(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Job::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Update an existing job by ID in the given namespace (CronJobUpdate).
    pub async fn update_job(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        job_id: &str,
        job: &JobCreateUpdate,
    ) -> Result<Job, tonic::Status> {
        let request = pb::CronJobUpdateRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            job_id: job_id.into(),
            job: Some(job.into()),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_job_update(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Job::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Delete a job by ID in the given namespace (CronJobDelete).
    pub async fn delete_job(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        job_id: &str,
    ) -> Result<(), tonic::Status> {
        let request = pb::CronJobDeleteRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            job_id: job_id.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_job_delete(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Ok(())
    }

    /// Retrieve a single job by its ID (CronJobGet).
    pub async fn get_job(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        job_id: &str,
    ) -> Result<Job, tonic::Status> {
        let request = pb::CronJobGetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            job_id: job_id.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_job_get(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Job::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Get the execution results for a single job by its ID (CronJobResults).
    pub async fn job_results(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        job_id: &str,
    ) -> Result<Vec<JobExecutionResult>, tonic::Status> {
        let request = pb::CronJobResultsRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            job_id: job_id.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_job_results(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .results
            .iter()
            .map(JobExecutionResult::try_from)
            .map(|result| result.map_err(conversion_error))
            .collect()
    }

    /// Retrieve a reference value by namespace and key (CronRefGet).
    pub async fn get_ref(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<Ref, tonic::Status> {
        let request = pb::CronRefGetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            key: key.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_ref_get(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Ref::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Set or update a reference value (CronRefSet).
    pub async fn set_ref(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        key: &str,
        reference: &RefCreateUpdate,
    ) -> Result<Ref, tonic::Status> {
        let request = pb::CronRefSetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            key: key.into(),
            r#ref: Some(reference.into()),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_ref_set(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Ref::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// Remove a reference value by namespace and key (CronRefUnset).
    pub async fn unset_ref(
        &mut self,
        tenant_id: &str,
        namespace: &str,
        key: &str,
    ) -> Result<(), tonic::Status> {
        let request = pb::CronRefUnsetRequest {
            tenant_id: tenant_id.into(),
            namespace: namespace.into(),
            key: key.into(),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cron_ref_unset(request).await }
            },
            retries,
            delay,
        )
        .await?;
        Ok(())
    }
}
