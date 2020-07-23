use serde::{Deserialize, Serialize};

pub const PER_PAGE: usize = 50;

pub type Id = i32;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostQuery {
    pub page: usize,
}

impl Default for PostQuery {
    fn default() -> Self {
        Self { page: 1 }
    }
}
