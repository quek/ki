use crate::common::types::{PostStatus, UserStatus};
#[cfg(target_arch = "x86_64")]
use crate::schema::*;
#[cfg(target_arch = "x86_64")]
use actix_identity::Identity;
#[cfg(target_arch = "x86_64")]
use actix_web::dev::Payload;
#[cfg(target_arch = "x86_64")]
use actix_web::{Error, FromRequest, HttpRequest};
#[cfg(target_arch = "x86_64")]
use futures::future::Future;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "x86_64")]
use std::pin::Pin;

pub const PER_PAGE: i64 = 50;

pub type Id = i32;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(Identifiable, Queryable, Associations))]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub body: String,
    pub status: PostStatus,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostQuery {
    pub page: i64,
}

impl Default for PostQuery {
    fn default() -> Self {
        Self { page: 1 }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(Identifiable, Queryable, Associations))]
pub struct User {
    pub id: Id,
    pub email: String,
    pub name: String,
    pub status: UserStatus,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[cfg(target_arch = "x86_64")]
impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;
    fn from_request(request: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let future = Identity::from_request(request, payload);
        Box::pin(async move {
            if let Some(identity) = future.await?.identity() {
                let user: User = serde_json::from_str(&identity)?;
                return Ok(user);
            };
            Err(actix_http::error::ErrorUnauthorized("Unauthorized"))
        })
    }
}
