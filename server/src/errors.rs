use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not found")]
    NotFound,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not found"),
        }
    }
}

impl From<actix_web::error::BlockingError<ServiceError>> for ServiceError {
    fn from(error: actix_web::error::BlockingError<ServiceError>) -> ServiceError {
        match error {
            actix_web::error::BlockingError::Error(service_error) => service_error,
            actix_web::error::BlockingError::Canceled => ServiceError::InternalServerError,
        }
    }
}

impl From<anyhow::Error> for ServiceError {
    fn from(error: anyhow::Error) -> ServiceError {
        log::error!("{:#?}", &error);
        ServiceError::InternalServerError
    }
}
