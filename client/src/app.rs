use crate::component;
use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::router::Router;

pub struct Model {}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| match switch {
            AppRoute::Index => {
                html! {
                  <component::index::Model />
                }
            }
            AppRoute::PostsShow(id) => {
                html! {
                <component::posts::show::Model id=id />
                }
            }
            AppRoute::Admin(route) => {
                html! {
                  <component::admin::Model route=route />
                }
            }
        });
        html! {
          <div>
            <Router<AppRoute> render=render />
            <component::loading::Model />
            <div id="toast"></div>
          </div>
        }
    }
}
