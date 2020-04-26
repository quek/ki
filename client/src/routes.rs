use crate::common::dto::Id;
use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/admin{*:rest}"]
    Admin(AdminRoute),
    #[to = "/posts/{id}"]
    PostsShow(Id),
    #[to = "/"]
    Index,
}

#[derive(Switch, Debug, Clone)]
pub enum AdminRoute {
    #[to = "/posts/new"]
    PostsNew,
    #[to = "/posts/{id}/edit"]
    PostsEdit(Id),
    #[to = ""]
    Index,
}
