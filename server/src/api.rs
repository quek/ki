use crate::errors::ServiceError;
use actix_web::HttpResponse;

pub mod admin;
pub mod posts;

pub async fn hello() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(format!(
        "ねこ {}",
        chrono::Local::now().format("%Y/%m/%d %H:%M:%S")
    )))
}
