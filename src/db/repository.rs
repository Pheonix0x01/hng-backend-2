use crate::error::ApiError;
use crate::models::{Country, CountryInsert, RefreshMetadata};
use chrono::Utc;
use sqlx::{MySql, Transaction};

pub async fn find_by_name(
    pool: &sqlx::Pool<MySql>,
    name: &str,
) -> Result<Option<Country>, ApiError> {
    let country = sqlx::query_as::<_, Country>(
        "SELECT id, name, capital, region, population, currency_code, exchange_rate, estimated_gdp, flag_url, last_refreshed_at FROM countries WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(country)
}

pub async fn find_by_name_case_insensitive(
    tx: &mut Transaction<'_, MySql>,
    name: &str,
) -> Result<Option<Country>, ApiError> {
    let country = sqlx::query_as::<_, Country>(
        "SELECT id, name, capital, region, population, currency_code, exchange_rate, estimated_gdp, flag_url, last_refreshed_at FROM countries WHERE LOWER(name) = LOWER(?)"
    )
    .bind(name)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(country)
}

pub async fn find_all(
    pool: &sqlx::Pool<MySql>,
    region: Option<String>,
    currency: Option<String>,
    sort: Option<String>,
) -> Result<Vec<Country>, ApiError> {
    let mut query_parts = vec![
        "SELECT id, name, capital, region, population, currency_code, exchange_rate, estimated_gdp, flag_url, last_refreshed_at FROM countries WHERE 1=1".to_string()
    ];

    let mut bindings: Vec<String> = Vec::new();

    if let Some(r) = region {
        query_parts.push("AND region = ?".to_string());
        bindings.push(r);
    }

    if let Some(c) = currency {
        query_parts.push("AND currency_code = ?".to_string());
        bindings.push(c);
    }

    if let Some(sort_param) = sort {
        match sort_param.as_str() {
            "gdp_desc" => query_parts.push("ORDER BY estimated_gdp DESC".to_string()),
            "gdp_asc" => query_parts.push("ORDER BY estimated_gdp ASC".to_string()),
            "population_desc" => query_parts.push("ORDER BY population DESC".to_string()),
            "population_asc" => query_parts.push("ORDER BY population ASC".to_string()),
            _ => {}
        }
    }

    let query = query_parts.join(" ");

    let mut q = sqlx::query_as::<_, Country>(&query);

    for binding in bindings {
        q = q.bind(binding);
    }

    let countries = q.fetch_all(pool).await?;

    Ok(countries)
}

pub async fn insert(
    tx: &mut Transaction<'_, MySql>,
    country: &CountryInsert,
) -> Result<(), ApiError> {
    sqlx::query(
        "INSERT INTO countries (name, capital, region, population, currency_code, exchange_rate, estimated_gdp, flag_url, last_refreshed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&country.name)
    .bind(&country.capital)
    .bind(&country.region)
    .bind(country.population)
    .bind(&country.currency_code)
    .bind(country.exchange_rate)
    .bind(country.estimated_gdp)
    .bind(&country.flag_url)
    .bind(Utc::now())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn update(
    tx: &mut Transaction<'_, MySql>,
    country: &CountryInsert,
) -> Result<(), ApiError> {
    sqlx::query(
        "UPDATE countries SET capital = ?, region = ?, population = ?, currency_code = ?, exchange_rate = ?, estimated_gdp = ?, flag_url = ?, last_refreshed_at = ? WHERE LOWER(name) = LOWER(?)"
    )
    .bind(&country.capital)
    .bind(&country.region)
    .bind(country.population)
    .bind(&country.currency_code)
    .bind(country.exchange_rate)
    .bind(country.estimated_gdp)
    .bind(&country.flag_url)
    .bind(Utc::now())
    .bind(&country.name)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn delete(
    pool: &sqlx::Pool<MySql>,
    name: &str,
) -> Result<bool, ApiError> {
    let result = sqlx::query("DELETE FROM countries WHERE name = ?")
        .bind(name)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_top_by_gdp(
    pool: &sqlx::Pool<MySql>,
    limit: i32,
) -> Result<Vec<Country>, ApiError> {
    let countries = sqlx::query_as::<_, Country>(
        "SELECT id, name, capital, region, population, currency_code, exchange_rate, estimated_gdp, flag_url, last_refreshed_at FROM countries WHERE estimated_gdp IS NOT NULL ORDER BY estimated_gdp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(countries)
}

pub async fn update_metadata(
    tx: &mut Transaction<'_, MySql>,
    total_countries: i32,
) -> Result<(), ApiError> {
    sqlx::query(
        "UPDATE refresh_metadata SET total_countries = ?, last_refreshed_at = ? WHERE id = 1"
    )
    .bind(total_countries)
    .bind(Utc::now())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_metadata(
    pool: &sqlx::Pool<MySql>,
) -> Result<RefreshMetadata, ApiError> {
    let metadata = sqlx::query_as::<_, RefreshMetadata>(
        "SELECT total_countries, last_refreshed_at FROM refresh_metadata WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    Ok(metadata)
}