use crate::component;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            <component::posts::index::Model />
            <component::login::Model />
          </>
        }
    }
}
