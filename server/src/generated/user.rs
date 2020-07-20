#[cfg(target_arch = "x86_64")]
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(
    target_arch = "x86_64",
    derive(FromSql, ToSql),
    postgres(name = "user_status")
)]
pub enum UserStatu {
    #[cfg_attr(target_arch = "x86_64", postgres(name = "active"))]
    Active,
    #[cfg_attr(target_arch = "x86_64", postgres(name = "locked"))]
    Locked,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub status: UserStatu,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserNew {
    pub id: Option<i32>,
    pub email: String,
    pub name: String,
    pub status: UserStatu,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
