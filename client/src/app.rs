use crate::routes::AppRoute;
use crate::{component, utils};
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

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            web_sys::console::log_1(&format!("route {:?} !!!!!!!!!!!!!!!!!!", &switch).into());
            match switch {
                AppRoute::Index => {
                    utils::set_title("記");
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
                    utils::set_title("管理画面");
                    html! {
                      <component::admin::Model route=route />
                    }
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
