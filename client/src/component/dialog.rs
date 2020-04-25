use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct Model {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Close,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub open: bool,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.props.open = false;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let close = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Close
        });
        if self.props.open {
            html! {
            <>
              <a class="dialog-backgroud" href="#" onclick=close></a>
              <div class="dialog">
                {self.props.children.iter().collect::<VNode>()}
              </div>
            </>
            }
        } else {
            html! { {""} }
        }
    }
}
