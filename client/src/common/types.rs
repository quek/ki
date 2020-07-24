use crate::generated::post::PostStatus;
use crate::generated::user::UserStatus;
use std::fmt;
use std::str;

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
