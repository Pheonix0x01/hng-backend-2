use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub id: u64,
    pub name: String,
    pub capital: Option<String>,
    pub region: Option<String>,
    pub population: i64,
    pub currency_code: Option<String>,
    pub exchange_rate: Option<f64>,
    pub estimated_gdp: Option<f64>,
    pub flag_url: Option<String>,
    pub last_refreshed_at: DateTime<Utc>,
}

impl FromRow<'_, sqlx::mysql::MySqlRow> for Country {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let naive_dt: NaiveDateTime = row.try_get("last_refreshed_at")?;
        let dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
        
        Ok(Country {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            capital: row.try_get("capital")?,
            region: row.try_get("region")?,
            population: row.try_get("population")?,
            currency_code: row.try_get("currency_code")?,
            exchange_rate: row.try_get("exchange_rate")?,
            estimated_gdp: row.try_get("estimated_gdp")?,
            flag_url: row.try_get("flag_url")?,
            last_refreshed_at: dt,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CountryInsert {
    pub name: String,
    pub capital: Option<String>,
    pub region: Option<String>,
    pub population: i64,
    pub currency_code: Option<String>,
    pub exchange_rate: Option<f64>,
    pub estimated_gdp: Option<f64>,
    pub flag_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshMetadata {
    pub total_countries: i32,
    pub last_refreshed_at: DateTime<Utc>,
}

impl FromRow<'_, sqlx::mysql::MySqlRow> for RefreshMetadata {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let naive_dt: NaiveDateTime = row.try_get("last_refreshed_at")?;
        let dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
        
        Ok(RefreshMetadata {
            total_countries: row.try_get("total_countries")?,
            last_refreshed_at: dt,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CountryApiResponse {
    pub name: String,
    pub capital: Option<String>,
    pub region: Option<String>,
    pub population: i64,
    pub flag: Option<String>,
    pub currencies: Option<Vec<Currency>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    pub code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeRateApiResponse {
    pub rates: HashMap<String, f64>,
}