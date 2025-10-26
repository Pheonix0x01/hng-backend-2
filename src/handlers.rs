use crate::config::Config;
use crate::db::{repository, DbPool};
use crate::error::ApiError;
use crate::services::CountryService;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct CountryQuery {
    region: Option<String>,
    currency: Option<String>,
    sort: Option<String>,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    message: String,
    total_countries: i32,
    last_refreshed_at: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    message: String,
}

#[post("/countries/refresh")]
async fn refresh_countries(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> Result<impl Responder, ApiError> {
    let service = CountryService::new(config.external_api_timeout_secs);
    
    let (total_countries, last_refreshed_at) = service.refresh_countries(&pool).await?;

    Ok(HttpResponse::Ok().json(RefreshResponse {
        message: format!("Successfully refreshed {} countries", total_countries),
        total_countries,
        last_refreshed_at: last_refreshed_at.to_rfc3339(),
    }))
}

#[get("/countries")]
async fn get_countries(
    pool: web::Data<DbPool>,
    query: web::Query<CountryQuery>,
) -> Result<impl Responder, ApiError> {
    let countries = repository::find_all(
        &pool,
        query.region.clone(),
        query.currency.clone(),
        query.sort.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(countries))
}

#[get("/countries/image")]
async fn get_summary_image() -> HttpResponse {
    let path = PathBuf::from("cache/summary.png");

    if !path.exists() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Summary image not found"
        }));
    }

    match std::fs::read(&path) {
        Ok(bytes) => {
            HttpResponse::Ok()
                .content_type("image/png")
                .body(bytes)
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal server error"
        })),
    }
}

#[get("/countries/{name}")]
async fn get_country_by_name(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let country = repository::find_by_name(&pool, &name)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(country))
}

#[delete("/countries/{name}")]
async fn delete_country(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let deleted = repository::delete(&pool, &name).await?;

    if !deleted {
        return Err(ApiError::NotFound);
    }

    Ok(HttpResponse::Ok().json(DeleteResponse {
        message: "Country deleted successfully".to_string(),
    }))
}

#[get("/status")]
async fn get_status(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let metadata = repository::get_metadata(&pool).await?;

    Ok(HttpResponse::Ok().json(metadata))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(refresh_countries)
        .service(get_countries)
        .service(get_summary_image)
        .service(get_country_by_name)
        .service(delete_country)
        .service(get_status);
}