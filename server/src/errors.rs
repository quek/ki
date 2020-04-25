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

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            diesel::result::Error::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                log::error!(
                    "diesel::result::Error kind: {:?}, message: {}",
                    kind,
                    message
                );
                if let diesel::result::DatabaseErrorKind::UniqueViolation = kind {
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            diesel::result::Error::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}
