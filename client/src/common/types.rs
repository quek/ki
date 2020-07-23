use crate::generated::post::PostStatus;
#[cfg(target_arch = "x86_64")]
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(target_arch = "x86_64", derive(DbEnum), DieselType = "User_status")]
pub enum UserStatus {
    Active,
    Locked,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Locked => write!(f, "Locked"),
        }
    }
}

impl str::FromStr for UserStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(UserStatus::Active),
            "Locked" => Ok(UserStatus::Locked),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PostStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "Draft"),
            Self::Published => write!(f, "Published"),
        }
    }
}

impl std::str::FromStr for PostStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Draft" => Ok(PostStatus::Draft),
            "Published" => Ok(PostStatus::Published),
            _ => Err(()),
        }
    }
}
