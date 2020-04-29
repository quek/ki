use crate::component::Link;
use crate::routes::{AdminRoute, AppRoute};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

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
        html! {
          <div id="header">
            <Link route=AppRoute::Index>{"トップ"}</Link>
            {" "}
            <Link route=AppRoute::Admin(AdminRoute::Posts)>{"記事"}</Link>
          </div>
        }
    }
}
