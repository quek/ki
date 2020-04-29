pub mod admin;
pub mod error;
pub mod pager;
// pub mod dialog;
pub mod index;
pub mod loading;
pub mod login;
pub mod posts;

use crate::routes::AppRoute;
use yew_router::components::RouterAnchor;

pub type Link = RouterAnchor<AppRoute>;
