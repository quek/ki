use crate::global_state::{GlobalStateAgent, Response};
use yew::agent::Bridged;
use yew::{html, Bridge, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    loading: bool,
    #[allow(dead_code)]
    global_state_agent: Box<dyn Bridge<GlobalStateAgent>>,
}

pub enum Msg {
    StartLoading,
    StopLoading,
    Nop,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|response: Response| match response {
            Response::StartLoading => Msg::StartLoading,
            Response::StopLoading => Msg::StopLoading,
            _ => Msg::Nop,
        });
        let global_state_agent = GlobalStateAgent::bridge(callback);
        Self {
            loading: false,
            global_state_agent,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartLoading => {
                self.loading = true;
                true
            }
            Msg::StopLoading => {
                self.loading = false;
                true
            }
            Msg::Nop => false,
        }
    }

    fn view(&self) -> Html {
        if self.loading {
            html! { <div id="loading">{""}</div> }
        } else {
            html! { {""} }
        }
    }
}
