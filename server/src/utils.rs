lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set!");
}
