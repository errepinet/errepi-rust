/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Examples of using the `GenericRegsClient` client to interact with the
//! Errepi Net generic registries microservice over gRPC (GenericRegsService).
//!
//! Shows every RPC of the service, both without filter and with the optional
//! prefix search filter. Requires a live GenericRegsService on
//! `localhost:50052`.

use errepi_rs::conf::RegsClientConfiguration;
use errepi_rs::regs::GenericRegsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiate with a connection configuration (host and port) or use the default.
    let mut regs = GenericRegsClient::new(RegsClientConfiguration {
        port: 50052,
        ..RegsClientConfiguration::default()
    })
    .await?;

    // 1. Get application info.
    let info = regs.app_info().await?;
    println!("App info: {info:?}");

    // 2. List states (no filter).
    let states = regs.states_list(None).await?;
    println!("States (no filter): {states:?}");
    println!("States count: {}", states.len());

    // 3. List states, optional prefix search on Italian name.
    let states_filtered = regs.states_list(Some("Ita")).await?;
    println!("States (search='Ita'): {states_filtered:?}");
    println!("States filtered count: {}", states_filtered.len());

    // 4. List cities (no filter).
    let cities = regs.cities_list(None).await?;
    println!("Cities (no filter): {cities:?}");
    println!("Cities count: {}", cities.len());

    // 5. List cities, optional prefix search on municipality name.
    let cities_filtered = regs.cities_list(Some("Roma")).await?;
    println!("Cities (search='Roma'): {cities_filtered:?}");
    println!("Cities filtered count: {}", cities_filtered.len());

    // 6. List caps (no filter).
    let caps = regs.caps_list(None).await?;
    println!("Caps (no filter): {caps:?}");
    println!("Caps count: {}", caps.len());

    // 7. List caps, optional prefix search on postal code.
    let caps_filtered = regs.caps_list(Some("001")).await?;
    println!("Caps (search='001'): {caps_filtered:?}");
    println!("Caps filtered count: {}", caps_filtered.len());

    // 8. List provinces (no filter).
    let provinces = regs.provinces_list(None).await?;
    println!("Provinces (no filter): {provinces:?}");
    println!("Provinces count: {}", provinces.len());

    // 9. List provinces, optional prefix search on province name.
    let provinces_filtered = regs.provinces_list(Some("Roma")).await?;
    println!("Provinces (search='Roma'): {provinces_filtered:?}");
    println!("Provinces filtered count: {}", provinces_filtered.len());

    // 10. List regions (no filter).
    let regions = regs.regions_list(None).await?;
    println!("Regions (no filter): {regions:?}");
    println!("Regions count: {}", regions.len());

    // 11. List regions, optional prefix search on region name.
    let regions_filtered = regs.regions_list(Some("Lazio")).await?;
    println!("Regions (search='Lazio'): {regions_filtered:?}");
    println!("Regions filtered count: {}", regions_filtered.len());

    Ok(())
}
