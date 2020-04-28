use crate::common::models::User;
use crate::common::types::UserStatus;
use crate::errors::ServiceError;
use crate::thread_data::ThreadData;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::block;
use actix_web::{Error, FromRequest, ResponseError};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::{ok, Future, Ready};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct EnsureLogin;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S> for EnsureLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = EnsureLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(EnsureLoginMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct EnsureLoginMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for EnsureLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, service_request: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();
        Box::pin(async move {
            let thread_data = service_request.app_data::<ThreadData>().unwrap();

            let (r, mut pl) = service_request.into_parts();
            let user = User::from_request(&r, &mut pl).await?;
            let result = block::<_, _, ServiceError>(move || {
                use crate::schema::users;
                use diesel::dsl::{exists, select};
                let conn: &PgConnection = &thread_data.pool.get().unwrap();
                let exists = select(exists(
                    users::table
                        .filter(users::id.eq(user.id))
                        .filter(users::status.eq(UserStatus::Active)),
                ))
                .get_result(conn)?;
                if exists {
                    Ok(())
                } else {
                    Err(ServiceError::Unauthorized)
                }
            })
            .await;

            match ServiceRequest::from_parts(r, pl) {
                Ok(service_request) => match result {
                    Ok(_) => Ok(service.call(service_request).await?),
                    Err(error) => {
                        let service_error = ServiceError::from(error);
                        let response_error = service_error.error_response();
                        Ok(service_request.into_response(response_error.into_body()))
                    }
                },
                Err(_) => Err(Error::from(())),
            }
        })
    }
}
