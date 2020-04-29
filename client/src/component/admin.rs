use crate::component;
use crate::routes::AdminRoute;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub mod header;
pub mod index;
pub mod posts;

pub struct Model {
    props: Props,
}

pub enum Msg {}

#[derive(Clone, Properties)]
pub struct Props {
    pub route: AdminRoute,
    pub query: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <>
          <component::admin::header::Model />
          {self.view_routes()}
        </>
        }
    }
}

impl Model {
    fn view_routes(&self) -> Html {
        match self.props.route {
            AdminRoute::PostsNew => html! { <component::admin::posts::new::Model /> },
            AdminRoute::PostsEdit(id) => html! { <component::admin::posts::edit::Model id=id /> },
            AdminRoute::Posts => {
                html! { <component::admin::index::Model query=self.props.query.clone() /> }
            }
        }
    }
}
