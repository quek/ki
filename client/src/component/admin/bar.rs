use crate::routes::{AdminRoute, AppRoute};
use crate::utils;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterButton;

type AppRouterButton = RouterButton<AppRoute>;

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Hi,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hi => {
                utils::toast("はぁい！");
                false
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Hi);
        html! {
          <>
            <p>{"Bar です"}</p>
            <p><button onclick=onclick>{"Hi"}</button></p>
            <AppRouterButton route=AppRoute::Admin(AdminRoute::Index)>
              {"index へ"}
            </AppRouterButton>
          </>
        }
    }
}
