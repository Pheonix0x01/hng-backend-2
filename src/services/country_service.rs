use crate::db::repository;
use crate::error::ApiError;
use crate::models::{CountryApiResponse, CountryInsert, ExchangeRateApiResponse};
use crate::services::{ExternalApiService, ImageGenerator};
use rand::Rng;
use sqlx::{MySql, Pool};

pub struct CountryService {
    external_api: ExternalApiService,
}

impl CountryService {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            external_api: ExternalApiService::new(timeout_secs),
        }
    }

    pub async fn refresh_countries(
        &self,
        pool: &Pool<MySql>,
    ) -> Result<(i32, chrono::DateTime<chrono::Utc>), ApiError> {
        let (countries_data, rates_data) = self.external_api.fetch_all_data().await?;

        let mut tx = pool.begin().await?;

        for country_api in countries_data.iter() {
            let country_insert = self.process_country(country_api, &rates_data);

            let existing = repository::find_by_name_case_insensitive(&mut tx, &country_insert.name).await?;

            if existing.is_some() {
                repository::update(&mut tx, &country_insert).await?;
            } else {
                repository::insert(&mut tx, &country_insert).await?;
            }
        }

        let total_countries = countries_data.len() as i32;
        repository::update_metadata(&mut tx, total_countries).await?;

        tx.commit().await?;

        let metadata = repository::get_metadata(pool).await?;

        match self.generate_summary_image(pool).await {
            Ok(_) => log::info!("Summary image generated successfully"),
            Err(e) => log::error!("Failed to generate summary image: {:?}", e),
        }

        Ok((metadata.total_countries, metadata.last_refreshed_at))
    }

    fn process_country(
        &self,
        country_api: &CountryApiResponse,
        rates: &ExchangeRateApiResponse,
    ) -> CountryInsert {
        let mut rng = rand::thread_rng();

        let currency_code = country_api
            .currencies
            .as_ref()
            .and_then(|currencies| {
                if currencies.is_empty() {
                    None
                } else {
                    currencies[0].code.clone()
                }
            });

        let (exchange_rate, estimated_gdp) = if let Some(ref code) = currency_code {
            if let Some(&rate) = rates.rates.get(code) {
                let random_multiplier = rng.gen_range(1000.0..=2000.0);
                let gdp = (country_api.population as f64 * random_multiplier) / rate;
                (Some(rate), Some(gdp))
            } else {
                (None, None)
            }
        } else {
            (None, Some(0.0))
        };

        CountryInsert {
            name: country_api.name.clone(),
            capital: country_api.capital.clone(),
            region: country_api.region.clone(),
            population: country_api.population,
            currency_code,
            exchange_rate,
            estimated_gdp,
            flag_url: country_api.flag.clone(),
        }
    }

    async fn generate_summary_image(&self, pool: &Pool<MySql>) -> Result<(), ApiError> {
        log::info!("Generating summary image...");
        let top_countries = repository::get_top_by_gdp(pool, 5).await?;
        log::info!("Found {} top countries by GDP", top_countries.len());
        
        let metadata = repository::get_metadata(pool).await?;

        ImageGenerator::generate(
            &top_countries,
            metadata.total_countries,
            metadata.last_refreshed_at,
        )?;

        log::info!("Image saved to cache/summary.png");
        Ok(())
    }
}