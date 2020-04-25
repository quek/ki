use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Model {
    message: Option<String>,
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub message: Option<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            message: props.message,
        }
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.message = props.message;
        true
    }

    fn view(&self) -> Html {
        match &self.message {
            None => {
                html! { {""} }
            }
            Some(message) => {
                html! { <div class="error">{message}</div> }
            }
        }
    }
}
