/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Generic registries domain models.
//!
//! Domain models of the regs service.

use serde::{Deserialize, Serialize};

/// State (country) record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub id: Option<String>,
    pub nome_it: String,
    pub nome_en: String,
    pub stato_o_territorio: String,
    pub area_codice: i32,
    pub area_nome: String,
    pub continente_codice: i32,
    pub continente_nome: String,
    pub codice_istat: i32,
    pub codice_istat_genitore: Option<i32>,
    pub codice_iso_3166_alpha2: Option<String>,
    pub codice_iso_3166_alpha3: Option<String>,
    pub codice_iso_3166_alpha3_genitore: Option<String>,
    pub codice_m49: Option<String>,
    pub codice_min: Option<String>,
    pub codice_at: Option<String>,
}

/// Municipality (comune) record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct City {
    pub id: Option<String>,
    pub istat: i32,
    pub comune: String,
    pub cap: Option<String>,
    pub regione: String,
    pub provincia: String,
    pub cod_fisco: String,
    pub superficie: f64,
    pub state_istat: i32,
    pub codice_regione: i32,
}

/// Postal code (cap) record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cap {
    pub id: Option<String>,
    pub istat: i32,
    pub cap: String,
    pub citta: Option<String>,
    pub provincia: Option<String>,
    pub provincia_sigla: Option<String>,
    pub stato_it: Option<String>,
    pub stato_en: Option<String>,
}

/// Province record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Province {
    pub id: Option<String>,
    pub sigla: String,
    pub provincia: String,
    pub superficie: i32,
    pub num_comuni: i32,
    pub codice_regione: i32,
    pub istat_stato: i32,
    pub codice_istat: String,
}

/// Region record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Region {
    pub id: Option<String>,
    pub codice_regione: i32,
    pub nome_it: String,
    pub ripartizione_geografica: String,
}
