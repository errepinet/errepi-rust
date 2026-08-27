/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Examples of using the `CronConfigurator` client to interact with the
//! Errepi Net Cron microservice over gRPC (CronBridgeService).
//!
//! Mirrors `errepi-py` `examples/cron_example.py`. Requires a live
//! CronBridgeService on `localhost:50051`.

use std::collections::HashMap;
use std::time::Duration;

use chrono::{Duration as ChronoDuration, Utc};
use errepi_rs::conf::CronClientConfiguration;
use errepi_rs::cron::{
    http_job_type, CronConfigurator, CronConfiguration, HTTPJob, JobBodyType, JobCreateUpdate,
    JobFrequency, JobHttpMethod, RefCreateUpdate,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate with a connection configuration (host and port) or use the default.
    let mut cron =
        CronConfigurator::new(CronClientConfiguration::default()).await?;

    let tenant_id = "my-tenant";
    let namespace = "default";

    // 1. Get application info.
    let info = cron.app_info().await?;
    println!("App info: {info:?}");

    // 2. Set a configuration.
    let config_set = CronConfiguration {
        job_max_retries: 3,
        job_retry_delay_secs: 60,
        set_at: None,
    };
    let config_entry = cron.set_configuration(tenant_id, namespace, "main", &config_set).await?;
    println!("Set configuration: {config_entry:?}");

    // 3. Get a configuration.
    let config = cron.get_configuration(tenant_id, namespace, "main").await?;
    println!("Get configuration: {config:?}");

    // 4. Unset a configuration.
    cron.unset_configuration(tenant_id, namespace, "main").await?;
    println!("Configuration unset.");

    let job_type = http_job_type(HTTPJob {
        body: Some("{\"key\": \"test\"}".into()),
        body_type: Some(JobBodyType::Json),
        headers: Some(HashMap::from([(
            "Authorization".to_string(),
            "Bearer token".to_string(),
        )])),
        method: JobHttpMethod::Post,
        timeout_seconds: None,
        url: "https://example.com/api".into(),
        user_agent: None,
        valid_http_codes: None,
    });

    // 5. Create a job. Use ALWAYS UTC.
    let job_create = JobCreateUpdate {
        description: Some("Test job".into()),
        enabled: true,
        frequency: None,
        job_type: job_type.clone(),
        next_execution_dt: Utc::now() + ChronoDuration::hours(1),
        use_configuration: None,
    };
    let job = cron.create_job(tenant_id, namespace, &job_create).await?;
    println!("Created job: {job:?}");

    // 6. List jobs.
    let jobs = cron.list_jobs(tenant_id, namespace).await?;
    println!("Jobs: {jobs:?}");

    // 7. Update a job (requires a valid job_id).
    let job_id = job.id.as_deref().expect("job id");
    let job_update = JobCreateUpdate {
        description: Some("Test job updated".into()),
        enabled: true,
        frequency: Some(JobFrequency::Minute(5)),
        job_type: job_type.clone(),
        next_execution_dt: Utc::now() + ChronoDuration::hours(2),
        use_configuration: None,
    };
    let updated_job = cron.update_job(tenant_id, namespace, job_id, &job_update).await?;
    println!("Updated job: {updated_job:?}");

    // 8. Get a single job (requires a valid job_id).
    let single = cron.get_job(tenant_id, namespace, job_id).await?;
    println!("Single job: {single:?}");

    // 9. Get job execution results (requires a valid job_id).
    let results = cron.job_results(tenant_id, namespace, job_id).await?;
    println!("Job execution results: {results:?}");

    // 10. Delete a job (requires a valid job_id).
    cron.delete_job(tenant_id, namespace, job_id).await?;
    println!("Job deleted.");

    // 11. Set a reference.
    let reference = RefCreateUpdate {
        value: "my_value".into(),
    };
    let set_ref = cron.set_ref(tenant_id, namespace, "myref", &reference).await?;
    println!("Set ref: {set_ref:?}");

    // 12. Get a reference.
    let got_ref = cron.get_ref(tenant_id, namespace, "myref").await?;
    println!("Got ref: {got_ref:?}");

    // 13. Unset a reference.
    cron.unset_ref(tenant_id, namespace, "myref").await?;
    println!("Reference unset.");

    // Keep the process alive long enough for the gRPC calls to complete.
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(())
}
