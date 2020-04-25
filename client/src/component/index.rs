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
            <component::foo::Model />
            <component::login::Model />
            <component::posts::index::Model />
          </>
        }
    }
}
