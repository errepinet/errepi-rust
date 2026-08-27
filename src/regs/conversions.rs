/*
 * errepi-rs - Rust bindings for Errepi Net microservices
 *
 * Copyright © 2023-2026 Errepi Net S.R.L.
 * Author: Valerio Faiuolo <valerio.faiuolo@errepinet.it>
 *
 * Licensed under the MIT License. See the LICENSE file for details.
 */

//! Conversions between generic registries domain models and the
//! prost-generated messages of `protos/generic_regs.proto` (AppInfo included).
//!
//! Mirrors `errepi-py` `errepi/regs/__init__.py` `_parse_message` approach:
//! every message field maps to the domain model field of the same name.

use crate::error::ConversionError;
use crate::models::AppInfo;
use crate::pb::errepi_regs as pb;

use super::models::{Cap, City, Province, Region, State};

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

impl From<&State> for pb::State {
    fn from(state: &State) -> Self {
        Self {
            id: state.id.clone(),
            nome_it: state.nome_it.clone(),
            nome_en: state.nome_en.clone(),
            stato_o_territorio: state.stato_o_territorio.clone(),
            area_codice: state.area_codice,
            area_nome: state.area_nome.clone(),
            continente_codice: state.continente_codice,
            continente_nome: state.continente_nome.clone(),
            codice_istat: state.codice_istat,
            codice_istat_genitore: state.codice_istat_genitore,
            codice_iso_3166_alpha2: state.codice_iso_3166_alpha2.clone(),
            codice_iso_3166_alpha3: state.codice_iso_3166_alpha3.clone(),
            codice_iso_3166_alpha3_genitore: state.codice_iso_3166_alpha3_genitore.clone(),
            codice_m49: state.codice_m49.clone(),
            codice_min: state.codice_min.clone(),
            codice_at: state.codice_at.clone(),
        }
    }
}

impl TryFrom<&pb::State> for State {
    type Error = ConversionError;

    fn try_from(state: &pb::State) -> Result<Self, Self::Error> {
        Ok(Self {
            id: state.id.clone(),
            nome_it: state.nome_it.clone(),
            nome_en: state.nome_en.clone(),
            stato_o_territorio: state.stato_o_territorio.clone(),
            area_codice: state.area_codice,
            area_nome: state.area_nome.clone(),
            continente_codice: state.continente_codice,
            continente_nome: state.continente_nome.clone(),
            codice_istat: state.codice_istat,
            codice_istat_genitore: state.codice_istat_genitore,
            codice_iso_3166_alpha2: state.codice_iso_3166_alpha2.clone(),
            codice_iso_3166_alpha3: state.codice_iso_3166_alpha3.clone(),
            codice_iso_3166_alpha3_genitore: state.codice_iso_3166_alpha3_genitore.clone(),
            codice_m49: state.codice_m49.clone(),
            codice_min: state.codice_min.clone(),
            codice_at: state.codice_at.clone(),
        })
    }
}

impl From<&City> for pb::City {
    fn from(city: &City) -> Self {
        Self {
            id: city.id.clone(),
            istat: city.istat,
            comune: city.comune.clone(),
            cap: city.cap.clone(),
            regione: city.regione.clone(),
            provincia: city.provincia.clone(),
            cod_fisco: city.cod_fisco.clone(),
            superficie: city.superficie,
            state_istat: city.state_istat,
            codice_regione: city.codice_regione,
        }
    }
}

impl TryFrom<&pb::City> for City {
    type Error = ConversionError;

    fn try_from(city: &pb::City) -> Result<Self, Self::Error> {
        Ok(Self {
            id: city.id.clone(),
            istat: city.istat,
            comune: city.comune.clone(),
            cap: city.cap.clone(),
            regione: city.regione.clone(),
            provincia: city.provincia.clone(),
            cod_fisco: city.cod_fisco.clone(),
            superficie: city.superficie,
            state_istat: city.state_istat,
            codice_regione: city.codice_regione,
        })
    }
}

impl From<&Cap> for pb::Cap {
    fn from(cap: &Cap) -> Self {
        Self {
            id: cap.id.clone(),
            istat: cap.istat,
            cap: cap.cap.clone(),
            citta: cap.citta.clone(),
            provincia: cap.provincia.clone(),
            provincia_sigla: cap.provincia_sigla.clone(),
            stato_it: cap.stato_it.clone(),
            stato_en: cap.stato_en.clone(),
        }
    }
}

impl TryFrom<&pb::Cap> for Cap {
    type Error = ConversionError;

    fn try_from(cap: &pb::Cap) -> Result<Self, Self::Error> {
        Ok(Self {
            id: cap.id.clone(),
            istat: cap.istat,
            cap: cap.cap.clone(),
            citta: cap.citta.clone(),
            provincia: cap.provincia.clone(),
            provincia_sigla: cap.provincia_sigla.clone(),
            stato_it: cap.stato_it.clone(),
            stato_en: cap.stato_en.clone(),
        })
    }
}

impl From<&Province> for pb::Province {
    fn from(province: &Province) -> Self {
        Self {
            id: province.id.clone(),
            sigla: province.sigla.clone(),
            provincia: province.provincia.clone(),
            superficie: province.superficie,
            num_comuni: province.num_comuni,
            codice_regione: province.codice_regione,
            istat_stato: province.istat_stato,
            codice_istat: province.codice_istat.clone(),
        }
    }
}

impl TryFrom<&pb::Province> for Province {
    type Error = ConversionError;

    fn try_from(province: &pb::Province) -> Result<Self, Self::Error> {
        Ok(Self {
            id: province.id.clone(),
            sigla: province.sigla.clone(),
            provincia: province.provincia.clone(),
            superficie: province.superficie,
            num_comuni: province.num_comuni,
            codice_regione: province.codice_regione,
            istat_stato: province.istat_stato,
            codice_istat: province.codice_istat.clone(),
        })
    }
}

impl From<&Region> for pb::Region {
    fn from(region: &Region) -> Self {
        Self {
            id: region.id.clone(),
            codice_regione: region.codice_regione,
            nome_it: region.nome_it.clone(),
            ripartizione_geografica: region.ripartizione_geografica.clone(),
        }
    }
}

impl TryFrom<&pb::Region> for Region {
    type Error = ConversionError;

    fn try_from(region: &pb::Region) -> Result<Self, Self::Error> {
        Ok(Self {
            id: region.id.clone(),
            codice_regione: region.codice_regione,
            nome_it: region.nome_it.clone(),
            ripartizione_geografica: region.ripartizione_geografica.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> State {
        State {
            id: Some("1".into()),
            nome_it: "Italia".into(),
            nome_en: "Italy".into(),
            stato_o_territorio: "Stato".into(),
            area_codice: 1,
            area_nome: "Europa".into(),
            continente_codice: 2,
            continente_nome: "Europa".into(),
            codice_istat: 100,
            codice_istat_genitore: None,
            codice_iso_3166_alpha2: Some("IT".into()),
            codice_iso_3166_alpha3: Some("ITA".into()),
            codice_iso_3166_alpha3_genitore: None,
            codice_m49: Some("380".into()),
            codice_min: Some("142".into()),
            codice_at: None,
        }
    }

    #[test]
    fn state_roundtrip() {
        let s = state();
        assert_eq!(State::try_from(&pb::State::from(&s)).unwrap(), s);
    }

    #[test]
    fn city_roundtrip() {
        let city = City {
            id: Some("10".into()),
            istat: 58091,
            comune: "Roma".into(),
            cap: Some("00100".into()),
            regione: "Lazio".into(),
            provincia: "Roma".into(),
            cod_fisco: "H501".into(),
            superficie: 1287.36,
            state_istat: 100,
            codice_regione: 12,
        };
        assert_eq!(City::try_from(&pb::City::from(&city)).unwrap(), city);
    }

    #[test]
    fn cap_roundtrip() {
        let cap = Cap {
            id: Some("100".into()),
            istat: 58091,
            cap: "00100".into(),
            citta: Some("Roma".into()),
            provincia: Some("Roma".into()),
            provincia_sigla: Some("RM".into()),
            stato_it: Some("Italia".into()),
            stato_en: Some("Italy".into()),
        };
        assert_eq!(Cap::try_from(&pb::Cap::from(&cap)).unwrap(), cap);
    }

    #[test]
    fn province_roundtrip() {
        let province = Province {
            id: Some("58".into()),
            sigla: "RM".into(),
            provincia: "Roma".into(),
            superficie: 5361,
            num_comuni: 121,
            codice_regione: 12,
            istat_stato: 100,
            codice_istat: "058".into(),
        };
        assert_eq!(
            Province::try_from(&pb::Province::from(&province)).unwrap(),
            province
        );
    }

    #[test]
    fn region_roundtrip() {
        let region = Region {
            id: Some("12".into()),
            codice_regione: 12,
            nome_it: "Lazio".into(),
            ripartizione_geografica: "Centro".into(),
        };
        assert_eq!(
            Region::try_from(&pb::Region::from(&region)).unwrap(),
            region
        );
    }

    #[test]
    fn app_info_roundtrip() {
        let info = AppInfo {
            name: "regs".into(),
            version: "0.1.0".into(),
            build_timestamp: "123".into(),
            build_date: "2024-01-01".into(),
            build_time: "10:00:00 UTC".into(),
            build_datetime: "2024-01-01 10:00:00 UTC".into(),
            git_hash: "abc".into(),
            git_branch: "main".into(),
        };
        assert_eq!(AppInfo::try_from(&pb::AppInfo::from(&info)).unwrap(), info);
    }
}
