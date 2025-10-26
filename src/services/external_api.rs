use crate::error::ApiError;
use crate::models::{CountryApiResponse, ExchangeRateApiResponse};
use reqwest::Client;
use std::time::Duration;

pub struct ExternalApiService {
    client: Client,
}

impl ExternalApiService {
    pub fn new(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    pub async fn fetch_countries(&self) -> Result<Vec<CountryApiResponse>, ApiError> {
        let url = "https://restcountries.com/v2/all?fields=name,capital,region,population,flag,currencies";
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|_| ApiError::ExternalApiError("restcountries.com".to_string()))?;

        if !response.status().is_success() {
            return Err(ApiError::ExternalApiError("restcountries.com".to_string()));
        }

        response
            .json::<Vec<CountryApiResponse>>()
            .await
            .map_err(|_| ApiError::ExternalApiError("restcountries.com".to_string()))
    }

    pub async fn fetch_exchange_rates(&self) -> Result<ExchangeRateApiResponse, ApiError> {
        let url = "https://open.er-api.com/v6/latest/USD";
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|_| ApiError::ExternalApiError("open.er-api.com".to_string()))?;

        if !response.status().is_success() {
            return Err(ApiError::ExternalApiError("open.er-api.com".to_string()));
        }

        response
            .json::<ExchangeRateApiResponse>()
            .await
            .map_err(|_| ApiError::ExternalApiError("open.er-api.com".to_string()))
    }

    pub async fn fetch_all_data(
        &self,
    ) -> Result<(Vec<CountryApiResponse>, ExchangeRateApiResponse), ApiError> {
        let countries = self.fetch_countries().await?;
        let rates = self.fetch_exchange_rates().await?;
        Ok((countries, rates))
    }
}