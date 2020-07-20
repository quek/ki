#[cfg(target_arch = "x86_64")]
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(
    target_arch = "x86_64",
    derive(FromSql, ToSql),
    postgres(name = "post_status")
)]
pub enum PostStatu {
    #[cfg_attr(target_arch = "x86_64", postgres(name = "draft"))]
    Draft,
    #[cfg_attr(target_arch = "x86_64", postgres(name = "published"))]
    Published,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub status: PostStatu,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostNew {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub status: PostStatu,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
