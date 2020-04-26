use crate::fetch;
use crate::utils;
use web_sys::MouseEvent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    fetch_task: Option<fetch::FetchTask>,
}

pub enum Msg {
    Login(MouseEvent),
    Location(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login(event) => {
                event.prevent_default();
                let callback = self
                    .link
                    .callback(move |location: String| Msg::Location(location));
                let fetch_task = fetch::FetchService::new().get("/api/login", callback);
                self.fetch_task = Some(fetch_task);
                false
            }
            Msg::Location(location) => {
                utils::set_location(&location);
                false
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|event| Msg::Login(event));
        html! {
          <div style="float: right;">
            <a href="/login" onclick=onclick>{"_"}</a>
          </div>
        }
    }
}
