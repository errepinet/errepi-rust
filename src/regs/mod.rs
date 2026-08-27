/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Client for the Errepi Net generic registries microservice
//! (GenericRegsService).
//!
//! Mirrors `errepi-py` `errepi/regs/__init__.py`.

pub mod conversions;
pub mod models;

use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::conf::RegsClientConfiguration;
use crate::error::ConversionError;
use crate::models::AppInfo;
use crate::pb::errepi_regs as pb;
use crate::pb::errepi_regs::generic_regs_service_client::GenericRegsServiceClient;
use crate::retry;

pub use models::{Cap, City, Province, Region, State};

fn conversion_error(error: ConversionError) -> tonic::Status {
    tonic::Status::internal(format!("conversion failed: {error}"))
}

/// Client for interacting with the Errepi Net generic registries microservice
/// (GenericRegsService).
///
/// Provides methods to retrieve application info and search states, cities,
/// caps, provinces and regions over gRPC. The interface mirrors the RPCs of
/// `protos/generic_regs.proto`.
#[derive(Clone)]
pub struct GenericRegsClient {
    stub: GenericRegsServiceClient<Channel>,
    config: RegsClientConfiguration,
}

impl GenericRegsClient {
    /// Connect to the generic registries microservice at the configured
    /// `host:port` (defaults to `localhost:50051`).
    pub async fn new(config: RegsClientConfiguration) -> Result<Self, tonic::transport::Error> {
        let endpoint = format!("http://{}:{}", config.host, config.port);
        let channel = Endpoint::from_shared(endpoint)?
            .connect()
            .await?;
        Ok(Self {
            stub: GenericRegsServiceClient::new(channel),
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
                async move { client.get_app_info(tonic::Request::new(())).await }
            },
            retries,
            delay,
        )
        .await?;
        AppInfo::try_from(&response.into_inner()).map_err(conversion_error)
    }

    /// List states, optional prefix search on Italian name (StatesList).
    pub async fn states_list(
        &mut self,
        search: Option<&str>,
    ) -> Result<Vec<State>, tonic::Status> {
        let request = pb::StatesListRequest {
            search: search.map(str::to_string),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.states_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .states
            .iter()
            .map(State::try_from)
            .map(|state| state.map_err(conversion_error))
            .collect()
    }

    /// List cities, optional prefix search on municipality name (CitiesList).
    pub async fn cities_list(
        &mut self,
        search: Option<&str>,
    ) -> Result<Vec<City>, tonic::Status> {
        let request = pb::CitiesListRequest {
            search: search.map(str::to_string),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.cities_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .cities
            .iter()
            .map(City::try_from)
            .map(|city| city.map_err(conversion_error))
            .collect()
    }

    /// List caps, optional prefix search on postal code (CapsList).
    pub async fn caps_list(&mut self, search: Option<&str>) -> Result<Vec<Cap>, tonic::Status> {
        let request = pb::CapsListRequest {
            search: search.map(str::to_string),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.caps_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .caps
            .iter()
            .map(Cap::try_from)
            .map(|cap| cap.map_err(conversion_error))
            .collect()
    }

    /// List provinces, optional prefix search on province name
    /// (ProvincesList).
    pub async fn provinces_list(
        &mut self,
        search: Option<&str>,
    ) -> Result<Vec<Province>, tonic::Status> {
        let request = pb::ProvincesListRequest {
            search: search.map(str::to_string),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.provinces_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .provinces
            .iter()
            .map(Province::try_from)
            .map(|province| province.map_err(conversion_error))
            .collect()
    }

    /// List regions, optional prefix search on region name (RegionsList).
    pub async fn regions_list(
        &mut self,
        search: Option<&str>,
    ) -> Result<Vec<Region>, tonic::Status> {
        let request = pb::RegionsListRequest {
            search: search.map(str::to_string),
        };
        let retries = self.config.max_retries;
        let delay = Duration::from_secs(self.config.retry_delay_secs);
        let client = self.stub.clone();
        let response = retry::call_with_retry(
            || {
                let mut client = client.clone();
                let request = request.clone();
                async move { client.regions_list(request).await }
            },
            retries,
            delay,
        )
        .await?;
        response
            .into_inner()
            .regions
            .iter()
            .map(Region::try_from)
            .map(|region| region.map_err(conversion_error))
            .collect()
    }
}
